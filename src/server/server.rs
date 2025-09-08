use crate::oauth::token::OauthManager;
use http_body_util::Full;
use hyper::{
    Method, Request, Response, StatusCode,
    body::{Bytes, Incoming},
};
use hyper_util::rt::TokioIo;
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};
use std::{collections::HashMap, fs, io, net::SocketAddr, ops::Deref, sync::Arc};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_rustls::TlsAcceptor;
use url::Url;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

pub struct TokenManager {
    active_requests: HashMap<String, oneshot::Sender<String>>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            active_requests: HashMap::new(),
        }
    }

    pub fn send_token(
        &mut self,
        auth_token: String,
        state_token: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(s) = self.active_requests.remove(state_token) {
            s.send(auth_token)?;
        }
        Ok(())
    }

    pub fn new_token_request(&mut self, state_token: String) -> Option<oneshot::Receiver<String>> {
        let (s, r) = oneshot::channel();
        if let None = self.active_requests.get(&state_token) {
            self.active_requests.insert(state_token, s);
            Some(r)
        } else {
            None
        }
    }
}

#[derive(Clone)]
// An Executor that uses the tokio runtime.
pub struct TokioExecutor;

// Implement the `hyper::rt::Executor` trait for `TokioExecutor` so that it can be used to spawn
// tasks in the hyper runtime.
// An Executor allows us to manage execution of tasks which can help us improve the efficiency and
// scalability of the server.
impl<F> hyper::rt::Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn(fut);
    }
}

pub async fn run_server(
    addr: SocketAddr,
    oauth_manager: std::sync::Arc<std::sync::Mutex<OauthManager>>,
    cancel_token: tokio_util::sync::CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    // add these to the config file
    // Load public certificate.
    let certs = load_certs("test/cert/cert.pem")?;
    // Load private key.
    let key = load_private_key("test/cert/key.pem")?;

    log::info!("Serving on: https://{}", addr.to_string());
    // Create a TCP listener via tokio.
    let incoming = TcpListener::bind(&addr).await?;

    // Build TLS configuration.
    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| error(e.to_string()))?;
    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => return Ok(()),
            connection = incoming.accept() => {
                let (stream, _) = connection?;

                let tls_stream = tls_acceptor.accept(stream).await?;
                let io = TokioIo::new(tls_stream);

                let om = oauth_manager.clone();
                tokio::task::spawn(async move {
                    if let Err(err) = hyper::server::conn::http2::Builder::new(TokioExecutor)
                        .serve_connection(io, Svc::new(om))
                        .await {
                        log::warn!("Error serving connection: {}", err);
                    }
                });
            },
        };
    }
}

struct Svc {
    om: std::sync::Arc<std::sync::Mutex<OauthManager>>,
}

impl Svc {
    pub fn new(om: std::sync::Arc<std::sync::Mutex<OauthManager>>) -> Self {
        Self { om }
    }
}

impl hyper::service::Service<Request<Incoming>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let mut response = Response::new(Full::default());
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                let mut unwrapped_om = self.om.lock().unwrap();

                if unwrapped_om.has_token() {
                    *response.body_mut() = Full::from(format!("we have a token!",));
                } else {
                    let auth_url = {
                        let mut ctx = std::task::Context::from_waker(std::task::Waker::noop());
                        let mut t = std::pin::pin!(unwrapped_om.reset_auth_url());

                        loop {
                            match t.as_mut().poll(&mut ctx) {
                                std::task::Poll::Ready(v) => break v,
                                std::task::Poll::Pending => {
                                    log::info!("Waiting on new auth url");
                                    continue;
                                }
                            }
                        }
                    };
                    *response.body_mut() = Full::from(format!("auth: {}", auth_url,));
                }
            }
            (&Method::GET, "/oauth") => {
                let mut code = None;
                let mut state = None; // state must match the csrf_token

                if let Ok(qp) = Url::parse(&req.uri().to_string()) {
                    for (key, value) in qp.query_pairs() {
                        match key.deref() {
                            "code" => code = Some(value.to_string()),
                            "state" => state = Some(value.to_string()),
                            &_ => (),
                        }
                    }

                    if let (Some(code_p), Some(state_p)) = (code, state) {
                        if let Err(e) = self
                            .om
                            .lock()
                            .unwrap()
                            .token_manager()
                            .send_token(code_p.clone(), &state_p)
                        {
                            log::error!("Error when unlocking the token manager: {}", e);
                            std::process::exit(1);
                            // handle the error somehow
                        } else {
                            if let Err(_) = self
                                .om
                                .lock()
                                .unwrap()
                                .token_manager()
                                .send_token(code_p.clone(), &state_p)
                            {
                                *response.body_mut() =
                                    Full::from(format!("Failed to store token",));
                            } else {
                                // eventually we will have a nice HTML webpage
                                *response.body_mut() = Full::from(format!(
                                    "Sent the token!",
                                    //"code: '{}', session: '{}', state: '{}'",
                                    //code_p, session_p, state_p
                                ));
                            }
                        }
                    }
                }
            }
            // Catch-all 404.
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        };

        Box::pin(async { Ok(response) })
    }
}

// Load public certificate from file.
fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    rustls_pemfile::certs(&mut reader).collect()
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}

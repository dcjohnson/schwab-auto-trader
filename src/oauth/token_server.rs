use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::{fs, io, ops::Deref};

use http_body_util::{/* BodyExt, */ Full};
use hyper::body::{Bytes, Incoming};
use hyper::http::{Method, Request, Response, StatusCode};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
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

pub async fn run_server(
    port: u16,
    tm: std::sync::Arc<std::sync::Mutex<TokenManager>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);

    // add these to the config file
    // Load public certificate.
    let certs = load_certs("test/cert/cert.pem")?;
    // Load private key.
    let key = load_private_key("test/cert/key.pem")?;

    println!("Starting to serve on https://{}", addr);

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
        let (tcp_stream, _remote_addr) = incoming.accept().await?;

        let tls_acceptor = tls_acceptor.clone();
        let tm_clone = tm.clone();
        tokio::spawn(async move {
            let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                Ok(tls_stream) => tls_stream,
                Err(err) => {
                    eprintln!("failed to perform tls handshake: {err:#}");
                    return;
                }
            };
            if let Err(err) = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(tls_stream), Svc::new(tm_clone))
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}

struct Svc {
    tm: std::sync::Arc<std::sync::Mutex<TokenManager>>,
}

impl Svc {
    pub fn new(tm: std::sync::Arc<std::sync::Mutex<TokenManager>>) -> Self {
        Self { tm }
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
            // Help route.
            (&Method::GET, "/") => {
                let mut code = None;
                let mut session = None;
                let mut state = None; // state must match the csrf_token

                if let Ok(qp) = Url::parse(&req.uri().to_string()) {
                    for (key, value) in qp.query_pairs() {
                        match key.deref() {
                            "code" => code = Some(value.to_string()),
                            "session" => session = Some(value.to_string()),
                            "state" => state = Some(value.to_string()),
                            &_ => (),
                        }
                    }

                    if let (Some(code_p), Some(session_p), Some(state_p)) = (code, session, state) {
                        if let Err(e) = self.tm.lock().unwrap().send_token(code_p.clone(), &state_p)
                        {
                            println!("Error when unlocking the token manager: {}", e);
                            std::process::exit(1);
                            // handle the error somehow
                        } else {
                            // eventually we will have a nice HTML webpage
                            *response.body_mut() = Full::from(format!(
                                "code: '{}', session: '{}', state: '{}'",
                                code_p, session_p, state_p
                            ));
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

use crate::{
    oauth::token::OauthManager,
    schwab::{account_manager::AccountManager, client::SchwabClient},
    server::web_resources::files::{css, html},
};
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
use std::{fs, io, net::SocketAddr, ops::Deref, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use url::Url;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
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
    oauth_manager: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    account_manager: std::sync::Arc<tokio::sync::Mutex<AccountManager>>,
    cancel_token: tokio_util::sync::CancellationToken,
    cert_path: String,
    key_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    // add these to the config file
    // Load public certificate.
    let certs = load_certs(&cert_path)?;
    // Load private key.
    let key = load_private_key(&key_path)?;

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

    let renderer = html::Renderer::new()?;

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => return Ok(()),
            connection = incoming.accept() => {
                match connection {
                    Ok((stream, _)) => {
                        match tls_acceptor.accept(stream).await {
                            Ok(tls_stream) => {
                                tokio::task::spawn({
                                    let io = TokioIo::new(tls_stream);
                                    let om = oauth_manager.clone();
                                    let renderer = renderer.clone();

                                    async move {
                                        if let Err(err) = hyper::server::conn::http2::Builder::new(TokioExecutor)
                                            .serve_connection(io, Svc::new(om, renderer))
                                            .await {
                                                log::warn!("Error serving connection: {}", err);
                                        }
                                    }
                                });
                            },
                            Err(e) => log::warn!("Couldn't accept tls connection: {}", e),
                        }
                    },
                    Err(e) => log::warn!("Couldn't accept tcp connection: {}", e),
                }
            },
        };
    }
}

struct Svc {
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    renderer: html::Renderer,
}

impl Svc {
    pub fn new(
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        renderer: html::Renderer,
    ) -> Self {
        Self { om, renderer }
    }
}

impl hyper::service::Service<Request<Incoming>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let om = self.om.clone();
        let renderer = self.renderer.clone();
        Box::pin(async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => {
                    if let Some(Ok(token)) = om.lock().await.get_token() {
                        return Ok(Response::new(Full::from(format!(
                            "VOO: {:?}",
                            SchwabClient::new(token).get_accounts().await,
                        ))));
                    } else {
                        return Ok(Response::new(Full::from(
                            renderer
                                .oauth(&html::OauthArgs {
                                    oauth_url: om.lock().await.reset_auth_url(),
                                })
                                .unwrap(),
                        )));
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
                            match om
                                .lock()
                                .await
                                .token_manager()
                                .send_token(code_p.clone(), &state_p)
                            {
                                Ok(()) => {
                                    return Ok(Response::new(Full::from(renderer.oauth_return(&html::OauthReturnArgs {
                                        oauth_return_message: "Authorization Successful; click on the button below to return to the homepage.".to_string(),
                                    }).unwrap())));
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to send token for completion of oauth authentication: '{}'",
                                        e
                                    );
                                }
                            }
                        }
                    }
                    return Ok(Response::new(Full::from(renderer.oauth_return(&html::OauthReturnArgs {
                                        oauth_return_message: "Authorization Not Successful; click on the button below to return to the homepage.".to_string(),
                                    }).unwrap())));
                }
                (&Method::GET, "/static/css/main.css") => {
                    return Ok(Response::new(Full::from(css::MAIN)));
                }
                (&Method::GET, "/static/css/oauth.css") => {
                    return Ok(Response::new(Full::from(css::OAUTH)));
                }
                (&Method::GET, "/static/css/oauth_return.css") => {
                    return Ok(Response::new(Full::from(css::OAUTH_RETURN)));
                }
                // Catch-all 404.
                _ => {
                    return {
                        let mut r = Response::new(Full::default());
                        *r.status_mut() = StatusCode::NOT_FOUND;
                        Ok(r)
                    };
                }
            }
        })
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

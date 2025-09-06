use std::{
    collections::HashMap,
    fs, io,
    net::{Ipv4Addr, SocketAddr},
    ops::Deref,
    sync::Arc,
};

use crate::oauth::token::OauthManager;

use http_body_util::Full;

use hyper::body::Bytes;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

use std::convert::Infallible;
use hyper::{ StatusCode, Method};
use hyper::{
    body::{ Incoming},
    //http::{Method, Request, Response, StatusCode},
};
use hyper_util::{
    rt::{ TokioIo, TokioTimer},
    server::conn::auto::Builder,
};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};
use tokio::{ sync::oneshot};
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






async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
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
    port: u16,
    oauth_manager: std::sync::Arc<std::sync::Mutex<OauthManager>>,
    cancel_token: tokio_util::sync::CancellationToken,
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






// Bind to the port and listen for incoming TCP connections
   // let listener = TcpListener::bind(addr).await?;

    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client-server communication.
        //
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (stream, _) = incoming.accept().await?;

        let tls_stream = tls_acceptor.accept(stream).await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tls_stream);

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP/2 connection we just received
        // to finish
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP/2 with an executor and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = hyper::server::conn::http2::Builder::new(TokioExecutor)
                .serve_connection(io, service_fn(async |_: Request<hyper::body::Incoming>| -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}))


                //.serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }






 Ok(())




/*

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => return Ok(()),
            connection = incoming.accept() => {
                let (tcp_stream, _remote_addr) = connection?;
                let tls_acceptor = tls_acceptor.clone();
                let om_clone = oauth_manager.clone();
                tokio::spawn(async move {
                    let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                        Ok(tls_stream) => tls_stream,
                        Err(err) => {
                            eprintln!("failed to perform tls handshake: {err:#}");
                            return;
                        }
                    };
                    if let Err(err) = Builder::new(TokioExecutor::new())
                        .serve_connection(TokioIo::new(tls_stream), Svc::new(om_clone))
                        .await
                    {
                        eprintln!("failed to serve connection: {err:#}");
                    }
                });
            },
        }
    }
*/
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
                // here I need to generate the oauth url if I don't have a token
                //

                let mut unwrapped_om = self.om.lock().unwrap();

                if unwrapped_om.has_token() {
                    *response.body_mut() = Full::from(format!(
                        "we have a token!",
                    ));
                } else {
                    *response.body_mut() = Full::from(format!(
                            "auth: ", //unwrapped_om.reset_auth_url().await, 
                    ));
                }
            }
            (&Method::GET, "/oauth") => {
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
                        if let Err(e) = self
                            .om
                            .lock()
                            .unwrap()
                            .token_manager()
                            .send_token(code_p.clone(), &state_p)
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

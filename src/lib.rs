pub mod oauth;
pub mod schwab;
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

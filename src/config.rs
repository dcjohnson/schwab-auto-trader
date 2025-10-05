use crate::Error;
use serde::Deserialize;
use serde_json::Deserializer as jsonDe;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_address: String,
    pub token_file_path: String,
    pub bind_address: String,
    pub cert_path: String,
    pub key_path: String,
    pub account_number: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        Ok(Config::deserialize(&mut jsonDe::from_reader(
            fs::File::open(path)?,
        ))?)
    }
}

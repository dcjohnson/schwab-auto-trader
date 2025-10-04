use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer as jsonDe, Serializer as jsonSer, de::SliceRead};
use std::{fs, io::Write};

#[derive(Deserialize)]
pub struct Config {
    pub client_id: String, 
    pub client_secret: String, 
    pub redirect_address: String, 
    pub token_file_path: String, 
    pub bind_address: String, 
    pub cert_path: String, 
    pub key_path: String, 
}

impl Config {
pub fn load(path: &str) -> Result<Self, Error> {
    Ok(Config::deserialize(&mut jsonDe::from_reader(
        fs::File::open(path)?,
    ))?)
}
}

use std::{
    fs::File,
    collections::HashMap,
};

use crate::{
    Error,
    oauth::token::OauthTokenResponse, 
};

use serde::{
    Serialize,
    Deserialize,
};

use serde_json::{
    de::{SliceRead, Read},
    Serializer as jsonSer,
    Deserializer as jsonDe,
};
use base64::{
    Engine, 
    engine::general_purpose,
};

#[derive(Serialize, Deserialize)]
pub struct StorageBackend {
    tokens: HashMap<String, String>,
}

impl StorageBackend {
    fn new() -> Self {
        Self {
            tokens: HashMap::new(), 
        }
    }
}

pub struct TokenStorage {
        path: String, 
        backend: StorageBackend,
}

impl TokenStorage {
    pub fn new(path: String, backend: StorageBackend) -> Self {
        Self { path, backend }
    }

    pub fn load(path: String) -> Result<Self, Error> {
        
    }

    pub fn set_token(&mut self, id: String, token: &OauthTokenResponse) -> Result<(), Error> {
        let mut token_bytes: Vec<u8> = Vec::new();
        token.serialize(&mut jsonSer::pretty(&mut token_bytes))?;
        self.backend.tokens.insert(
            id, general_purpose::STANDARD.encode(token_bytes),
        );
        Ok(())
    }

    pub fn get_token(&self, id: &String) -> Option<Result<OauthTokenResponse, Error>> {
        match self.backend.tokens.get(id) {
            None => None, 
            Some(b64t) => Some({
                match general_purpose::STANDARD.decode(b64t) {
                    Ok(bytes) => match OauthTokenResponse::deserialize(&mut jsonDe::<SliceRead>::from_slice(&bytes)) {
                        Ok(t) => Ok(t), 
                        Err(e) => Err(Box::new(e)),
                    },
                    Err(e) => Err(Box::new(e)),
                }
            }),
        }
    }
}


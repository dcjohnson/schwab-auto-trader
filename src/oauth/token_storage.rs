use std::{collections::HashMap, fs};

use crate::{Error, oauth::token::OauthTokenResponse};

use serde::{Deserialize, Serialize};

use base64::{Engine, engine::general_purpose};
use serde_json::{Deserializer as jsonDe, Serializer as jsonSer, de::SliceRead};

#[derive(Serialize, Deserialize)]
pub struct StorageBackend {
    tokens: HashMap<String, String>,
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
        Ok(Self {
            backend: StorageBackend::deserialize(&mut jsonDe::from_reader(fs::File::open(&path)?))?,
            path,
        })
    }

    pub fn safe(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn set_token(&mut self, id: String, token: &OauthTokenResponse) -> Result<(), Error> {
        let mut token_bytes: Vec<u8> = Vec::new();
        token.serialize(&mut jsonSer::pretty(&mut token_bytes))?;
        self.backend
            .tokens
            .insert(id, general_purpose::STANDARD.encode(token_bytes));
        Ok(())
    }

    pub fn get_token(&self, id: &String) -> Option<Result<OauthTokenResponse, Error>> {
        match self.backend.tokens.get(id) {
            None => None,
            Some(b64t) => Some({
                match general_purpose::STANDARD.decode(b64t) {
                    Ok(bytes) => match OauthTokenResponse::deserialize(
                        &mut jsonDe::<SliceRead>::from_slice(&bytes),
                    ) {
                        Ok(t) => Ok(t),
                        Err(e) => Err(Box::new(e)),
                    },
                    Err(e) => Err(Box::new(e)),
                }
            }),
        }
    }
}

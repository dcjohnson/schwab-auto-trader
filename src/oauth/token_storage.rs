use std::{fs, io::Write};

use crate::{Error, oauth::token::OauthTokenResponse};

use serde::{Deserialize, Serialize};

use base64::{Engine, engine::general_purpose};
use serde_json::{Deserializer as jsonDe, Serializer as jsonSer, de::SliceRead};

#[derive(Serialize, Deserialize)]
pub struct StorageBackend {
    token: Option<String>,
}

impl StorageBackend {
    pub fn new() -> Self {
        Self { token: None }
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
        // open file, if file doesn't exist, create it and then return an empty StorageBackend
        let fd = match fs::File::open(&path) {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    // create file
                    fs::File::create(&path)?;
                    return Ok(Self {
                        backend: StorageBackend::new(),
                        path,
                    });
                }
                _ => return Err(e.into()),
            },
        };

        Ok(Self {
            backend: match StorageBackend::deserialize(&mut jsonDe::from_reader(fd)) {
                Ok(b) => b,
                Err(_) => StorageBackend::new(),
            },
            path,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        fs::File::create(self.path.clone())?.write_all(&serde_json::to_vec(&self.backend)?)?;
        Ok(())
    }

    pub fn set_token(&mut self, token: &OauthTokenResponse) -> Result<(), Error> {
        let mut token_bytes: Vec<u8> = Vec::new();
        token.serialize(&mut jsonSer::pretty(&mut token_bytes))?;
        self.backend.token = Some(general_purpose::STANDARD.encode(token_bytes));
        self.save()?;
        Ok(())
    }

    pub fn has_token(&self) -> bool {
        match self.backend.token {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get_token(&self) -> Option<Result<OauthTokenResponse, Error>> {
        self.backend.token.clone().map(|b64t| {
            Some({
                match general_purpose::STANDARD.decode(b64t) {
                    Ok(bytes) => match OauthTokenResponse::deserialize(
                        &mut jsonDe::<SliceRead>::from_slice(&bytes),
                    ) {
                        Ok(t) => Ok(t),
                        Err(e) => Err(Box::new(e) as Error),
                    },
                    Err(e) => Err(Box::new(e) as Error),
                }
            })
        })?
    }
}

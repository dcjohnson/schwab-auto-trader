use crate::{Error, oauth::token::OauthTokenResponse};
use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer as jsonDe, Serializer as jsonSer, de::SliceRead};
use std::{fs, io::Write};

#[derive(Serialize, Deserialize)]
pub struct StorageBackend {
    token: Option<String>,
    expiration_timestamp: Option<String>,
}

impl StorageBackend {
    pub fn new() -> Self {
        Self {
            token: None,
            expiration_timestamp: None,
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

    pub fn set_token(
        &mut self,
        token: &OauthTokenResponse,
        expiration: DateTime<Utc>,
    ) -> Result<(), Error> {
        let mut token_bytes: Vec<u8> = Vec::new();
        token.serialize(&mut jsonSer::pretty(&mut token_bytes))?;
        self.backend.token = Some(general_purpose::STANDARD.encode(token_bytes));
        self.backend.expiration_timestamp = Some(expiration.to_rfc3339());
        self.save()?;
        Ok(())
    }

    pub fn has_token(&self) -> bool {
        match self.backend.token {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get_expiration(&self) -> Option<Result<DateTime<Utc>, Error>> {
        self.backend
            .expiration_timestamp
            .clone()
            .map(|ts| Ok(DateTime::parse_from_rfc3339(&ts)?.to_utc()))
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

    pub fn get_token_and_expiration(
        &self,
    ) -> Option<Result<(OauthTokenResponse, DateTime<Utc>), Error>> {
        match (self.get_token(), self.get_expiration()) {
            (Some(Ok(token)), Some(Ok(expir))) => Some(Ok((token, expir))),
            (None, _) => None,
            (_, None) => None,
            (Some(Err(e)), _) => Some(Err(e)),
            (_, Some(Err(e))) => Some(Err(e)),
        }
    }
}

use crate::Error;
use serde::Deserialize;
use serde_json::Deserializer as jsonDe;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_address: String,
    pub token_file_path: String,
    pub bind_address: String,
    pub cert_path: String,
    pub key_path: String,
    pub trading_config: TradingConfig,
}

impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        self.trading_config.validate()
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradingConfig {
    pub account_number: String,
    pub trading_collections: Vec<TradingCollection>,
    pub allocations: Vec<Allocation>,
}

impl TradingConfig {
    pub fn to_maps(&self) -> (HashMap<String, Vec<String>>, HashMap<String, f64>) {
        (
            self.trading_collections
                .iter()
                .fold(HashMap::new(), |mut m, v| {
                    m.insert(v.id.clone(), v.collection.clone());
                    m
                }),
            self.allocations.iter().fold(HashMap::new(), |mut m, v| {
                m.insert(v.id.clone(), v.allocation.clone());
                m
            }),
        )
    }
    pub fn validate(&self) -> Result<(), Error> {
        let (collections, allocations) = self.to_maps();

        let mut sum = 0.0;

        for (id, a) in allocations.iter() {
            if !collections.contains_key(id) {
                return Err(format!("'{}' is not a known collection", id).into());
            }

            sum += a;
        }

        if sum != 100.0 {
            return Err(format!("allocations don't added up to '{}%' not 100%", sum).into());
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradingCollection {
    pub id: String,
    pub collection: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Allocation {
    pub id: String,
    pub allocation: f64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        Ok(Config::deserialize(&mut jsonDe::from_reader(
            fs::File::open(path)?,
        ))?)
    }
}

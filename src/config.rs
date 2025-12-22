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
    pub allocations_percent: Vec<AllocationPercent>,
    pub allocations_amount: Vec<AllocationAmount>,

    // will be -infinity to infinity
    pub target_cash_balance: f64,
}

impl TradingConfig {
    pub fn to_maps(
        &self,
    ) -> (
        HashMap<String, Vec<String>>,
        HashMap<String, f64>,
        HashMap<String, u64>,
    ) {
        (
            self.trading_collections
                .iter()
                .fold(HashMap::new(), |mut m, v| {
                    m.insert(v.id.clone(), v.collection.clone());
                    m
                }),
            self.allocations_percent
                .iter()
                .fold(HashMap::new(), |mut m, v| {
                    m.insert(v.id.clone(), v.percent.clone());
                    m
                }),
            self.allocations_amount
                .iter()
                .fold(HashMap::new(), |mut m, v| {
                    m.insert(v.id.clone(), v.amount.clone());
                    m
                }),
        )
    }
    pub fn validate(&self) -> Result<(), Error> {
        let (collections, percents, amounts) = self.to_maps();

        let mut sum = 0.0;

        for (id, a) in percents.iter() {
            if !collections.contains_key(id) {
                return Err(format!("'{}' is not a known collection", id).into());
            }

            sum += a;
        }

        for id in amounts.keys() {
            if !collections.contains_key(id) {
                return Err(format!("'{}' is not a known collection", id).into());
            }
        }

        if sum != 100.0 {
            return Err(format!("allocations don't added up to '{}%' not 100%", sum).into());
        }

        Ok(())
    }
}

// A collection of stocks under an ID, suports multiple stocks for Tax Loss Harvesting in the
// Future.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradingCollection {
    pub id: String,
    pub collection: Vec<String>,
}

// The allocations per collection
#[derive(Deserialize, Debug, Default, Clone)]
pub struct AllocationPercent {
    pub id: String,

    pub percent: f64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AllocationAmount {
    pub id: String,

    pub amount: u64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        Ok(Config::deserialize(&mut jsonDe::from_reader(
            fs::File::open(path)?,
        ))?)
    }
}

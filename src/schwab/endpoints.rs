const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";
const TRADER_ENDPOINT: &str = "https://api.schwabapi.com/trader/v1";

pub fn accounts() -> String {
    format!("{}/{}?fields=positions", TRADER_ENDPOINT, "accounts")
}

pub fn ticker_quotes_data(ticker: &str) -> String {
    format!("{}/{}/{}", MARKET_DATA_ENDPOINT, ticker, "quotes")
}

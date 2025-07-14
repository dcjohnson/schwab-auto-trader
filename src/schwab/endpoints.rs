const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";

pub fn ticker_quotes_data(ticker: &str) -> String {
    format!("{}/{}/{}", MARKET_DATA_ENDPOINT, ticker, "quotes")
}

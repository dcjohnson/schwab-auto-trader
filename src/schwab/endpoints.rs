const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";
const TRADER_ENDPOINT: &str = "https://api.schwabapi.com/trader/v1";

pub fn accounts() -> String {
    format!("{}/accounts?fields=positions", TRADER_ENDPOINT)
}

pub fn account(account_number: &str) -> String {
    format!(
        "{}/accounts/{}?fields=positions",
        TRADER_ENDPOINT, account_number
    )
}

pub fn account_numbers() -> String {
    format!("{}/accounts/accountNumbers", TRADER_ENDPOINT)
}

pub fn ticker_quotes_data(ticker: &str) -> String {
    format!("{}/{}/quotes", MARKET_DATA_ENDPOINT, ticker)
}

pub fn transactions(account_number: &str) -> String {
    format!(
        "{}/accounts/{}/transactions",
        TRADER_ENDPOINT, account_number
    )
}

pub fn transaction(account_number: &str, transaction_id: &str) -> String {
    format!(
        "{}/accounts/{}/transactions/{}",
        TRADER_ENDPOINT, account_number, transaction_id
    )
}

use crate::schwab::schemas::accounts_and_trading::accounts::TransactionType;
use chrono::{DateTime, Utc, format::SecondsFormat};

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

pub fn transactions(
    account_number: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    transaction_type: TransactionType,
) -> String {
    format!(
        "{}/accounts/{}/transactions?startDate={}&endDate={}&types={}",
        TRADER_ENDPOINT,
        account_number,
        urlencoding::encode(&start_date.to_rfc3339_opts(SecondsFormat::Secs, true)),
        urlencoding::encode(&end_date.to_rfc3339_opts(SecondsFormat::Secs, true)),
        transaction_type
    )
}

pub fn transaction(account_number: &str, transaction_id: &str) -> String {
    format!(
        "{}/accounts/{}/transactions/{}",
        TRADER_ENDPOINT, account_number, transaction_id
    )
}

pub fn order(account_number: &str) -> String {
    format!("{}/accounts/{}/orders", TRADER_ENDPOINT, account_number)
}

pub fn orders(
    account_number: &str,
    from_entered_time: DateTime<Utc>,
    to_entered_time: DateTime<Utc>,
) -> String {
    format!(
        "{}/accounts/{}/orders?fromEnteredTime={}&toEnteredTime={}",
        TRADER_ENDPOINT,
        account_number,
        urlencoding::encode(&from_entered_time.to_rfc3339_opts(SecondsFormat::Secs, true)),
        urlencoding::encode(&to_entered_time.to_rfc3339_opts(SecondsFormat::Secs, true)),
    )
}

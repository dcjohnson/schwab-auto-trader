#![allow(dead_code)]
use serde::Deserialize;

pub type AccountNumbers = Vec<AccountNumber>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountNumber {
    pub account_number: String,
    pub hash_value: String,
}

pub type Accounts = Vec<AccountTypes>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountTypes {
    pub securities_account: Option<SecuritiesAccount>,
    cash_account: Option<CashAccount>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "assetType")]
pub enum AccountInstrument {
    #[serde(rename(deserialize = "COLLECTIVE_INVESTMENT"))]
    CollectiveInvestment {
        cusip: String,
        symbol: String,
        description: String,
        r#type: String,
    },

    #[serde(rename(deserialize = "OPTION"))]
    Option {
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
        r#type: String,

        #[serde(rename(deserialize = "putCall"))]
        put_call: String,

        #[serde(rename(deserialize = "underlyingSymbol"))]
        underlying_symbol: String,
    },

    #[serde(rename(deserialize = "EQUITY"))]
    Equity {
        cusip: String,
        symbol: String,

        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountApiOptionDeliverable {
    symbol: String,
    deliverable_units: f64,
    api_currency_type: String,
    asset_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    short_quantity: f64,
    average_price: f64,
    current_day_profit_loss: f64,
    current_day_profit_loss_percentage: f64,
    long_quantity: f64,
    settled_long_quantity: f64,
    settled_short_quantity: f64,
    aged_quantity: Option<f64>,
    instrument: AccountInstrument,
    market_value: f64,
    maintenance_requirement: Option<f64>,
    average_long_price: Option<f64>,
    average_short_price: Option<f64>,
    tax_log_average_price: Option<f64>,
    tax_log_average_short_price: Option<f64>,
    long_open_profit_loss: Option<f64>,
    short_open_profit_loss: Option<f64>,
    previous_session_long_quantity: Option<f64>,
    previous_session_short_quantity: Option<f64>,
    current_day_cost: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginInitialBalance {
    accrued_interest: f64,
    available_funds_non_marginable_trade: f64,
    bond_value: f64,
    buying_power: f64,
    cash_balance: f64,
    cash_available_for_trading: f64,
    cash_receipts: f64,
    day_trading_buying_power: Option<f64>,
    day_trading_buying_power_call: Option<f64>,
    day_trading_equity_call: Option<f64>,
    equity: Option<f64>,
    equity_percentage: Option<f64>,
    liquidation_value: f64,
    long_margin_value: Option<f64>,
    long_option_market_value: f64,
    long_stock_value: f64,
    maintenance_call: f64,
    maintenance_requirement: Option<f64>,
    margin: f64,
    margin_equity: f64,
    money_market_fund: f64,
    mutual_fund_value: f64,
    reg_t_call: f64,
    short_margin_value: Option<f64>,
    short_option_market_value: f64,
    short_stick_value: Option<f64>,
    total_cash: f64,
    is_in_call: bool,
    unsettled_cash: Option<f64>,
    pending_deposits: f64,
    margin_balance: Option<f64>,
    short_balance: Option<f64>,
    pub account_value: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalance {
    available_funds: f64,
    available_funds_non_marginable_trade: f64,
    buying_power: f64,
    buying_power_non_marginable_trade: Option<f64>,
    day_trading_buying_power: Option<f64>,
    day_trading_buying_power_call: Option<f64>,
    equity: Option<f64>,
    equity_percentage: Option<f64>,
    long_margin_value: Option<f64>,
    maintenance_call: f64,
    maintenance_requirement: Option<f64>,
    margin_balance: Option<f64>,
    reg_t_call: f64,
    short_balance: Option<f64>,
    short_margin_value: Option<f64>,
    sma: Option<f64>,
    is_in_call: Option<bool>,
    stock_buying_power: Option<f64>,
    option_buying_power: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    account_number: String,
    round_trips: i32,
    is_day_trader: bool,
    is_closing_only_restricted: bool,
    pfcb_flag: bool,
    pub positions: Option<Vec<Position>>,
    pub initial_balances: MarginInitialBalance,
    pub current_balances: MarginBalance,
    projected_balances: MarginBalance,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {}

pub type Transactions = Vec<Transaction>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
pub activity_id: i64, 
pub time: String, 
pub user: UserDetails, 
pub description: String, 
pub account_number: String ,
pub r#type: TransactionType, 
pub status: TransactionStatus,
pub sub_account: SubAccount,
pub tradeDate : 	String ,
pub settlementDate :	String, 
pub positionId :	i64, 
pub orderId	: i64 , 
pub netAmount: f64, 
pub activityType :	TransactionActivityType, 
pub transferItems: Vec<TransferItem>, 
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {} 

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubAccount {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionActivityType {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {}

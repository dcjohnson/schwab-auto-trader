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
#[serde(tag = "assetType")] // why is this here? 
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
    pub account_number: String,
    pub r#type: TransactionType,
    pub status: TransactionStatus,
    pub sub_account: SubAccount,
    pub trade_date: String,
    pub settlement_date: String,
    pub position_id: i64,
    pub order_id: i64,
    pub net_amount: f64,
    pub activity_type: TransactionActivityType,
    pub transfer_items: Vec<TransferItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {}

#[derive(Deserialize, Debug)]
pub enum TransactionType {
    #[serde(rename(deserialize = "TRADE"))]
    Trade,

    #[serde(rename(deserialize = "RECEIVE_AND_DELIVER"))]
    ReceiveAndDeliver,

    #[serde(rename(deserialize = "DIVIDEND_OR_INTEREST"))]
    DividendOrInterest,

    #[serde(rename(deserialize = "ACH_RECEIPT"))]
    AchReceipt,

    #[serde(rename(deserialize = "ACH_DISBURSEMENT"))]
    AchDisbursement,

    #[serde(rename(deserialize = "CASH_RECEIPT"))]
    CashReceipt,

    #[serde(rename(deserialize = "CASH_DISBURSEMENT"))]
    CashDisbursement,

    #[serde(rename(deserialize = "ELECTRONIC_FUND"))]
    ElectronicFund,

    #[serde(rename(deserialize = "WIRE_OUT"))]
    WireOut,

    #[serde(rename(deserialize = "WIRE_IN"))]
    WireIn,

    #[serde(rename(deserialize = "JOURNAL"))]
    Journal,

    #[serde(rename(deserialize = "MEMORANDUM"))]
    Memorandum,

    #[serde(rename(deserialize = "MARGIN_CALL"))]
    MarginCall,

    #[serde(rename(deserialize = "MONEY_MARKET"))]
    MoneyMarket,

    #[serde(rename(deserialize = "SMA_ADJUSTMENT"))]
    SmaAdjustment,
}

#[derive(Deserialize, Debug)]
pub enum TransactionStatus {
    #[serde(rename(deserialize = "VALID"))]
    Valid,

    #[serde(rename(deserialize = "INVALID"))]
    Invalid,

    #[serde(rename(deserialize = "PENDING"))]
    Pending,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum SubAccount {
    #[serde(rename(deserialize = "CASH"))]
    Cash,

    #[serde(rename(deserialize = "MARGIN"))]
    Margin,

    #[serde(rename(deserialize = "SHORT"))]
    Short,

    #[serde(rename(deserialize = "DIV"))]
    Div,

    #[serde(rename(deserialize = "INCOME"))]
    Income,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum TransactionActivityType {
    #[serde(rename(deserialize = "ACTIVITY_CORRECTION"))]
    ActivityCorrection,

    #[serde(rename(deserialize = "EXECUTION"))]
    Execution,

    #[serde(rename(deserialize = "ORDER_ACTION"))]
    OrderAction,

    #[serde(rename(deserialize = "TRANSFER"))]
    Transfer,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {}

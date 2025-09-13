use serde::Deserialize;

pub type Accounts = Vec<Account>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    securities_account: SecuritiesAccount,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountInstrument {
    AccountCashEquivalent {
        asset_type: String,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
        r#type: String,
    },
    AccountEquity {
        asset_type: String,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
    },
    AccountFixedIncome {
        asset_type: String,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
        maturity_date: String,
        factor: f64,
        variable_rate: f64,
    },
    AccountMutualFund {
        asset_type: String,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
    },
    AccountOption {
        asset_type: String,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,

        option_deliveries: Vec<AccountApiOptionDeliverable>,
        put_call: String,
        option_multiplier: i32,
        r#type: String,
        underlying_symbol: String,
    },
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
    aged_quantity: f64,
    instrument: AccountInstrument,
    market_value: f64,
    maintenance_requirement: f64,
    average_long_price: f64,
    average_short_price: f64,
    tax_log_average_price: f64,
    tax_log_average_short_price: f64,
    long_open_profit_loss: f64,
    short_open_profit_loss: f64,
    previous_session_long_quantity: f64,
    previous_session_short_quantity: f64,
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
    day_trading_buying_power: f64,
    day_trading_buying_power_call: f64,
    day_trading_equity_call: f64,
    equity: f64,
    equity_percentage: f64,
    liquidation_value: f64,
    long_margin_value: f64,
    long_option_market_value: f64,
    long_stock_value: f64,
    maintenance_call: f64,
    maintenance_requirement: f64,
    margin: f64,
    margin_equity: f64,
    money_market_fund: f64,
    mutual_fund_value: f64,
    reg_t_call: f64,
    short_margin_value: f64,
    short_option_market_value: f64,
    short_stick_value: f64,
    total_cash: f64,
    is_in_call: f64,
    unsettled_cash: f64,
    pending_deposits: f64,
    margin_balance: f64,
    short_balance: f64,
    account_value: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalance {
    available_funds: f64,
    available_funds_non_marginable_trade: f64,
    buying_power: f64,
    buying_power_non_marginable_trade: f64,
    day_trading_buying_power: f64,
    day_trading_buying_power_call: f64,
    equity: f64,
    equity_percentage: f64,
    long_margin_value: f64,
    maintenance_call: f64,
    maintenance_requirement: f64,
    margin_balance: f64,
    reg_t_call: f64,
    short_balance: f64,
    short_margin_value: f64,
    sma: f64,
    is_in_call: f64,
    stock_buying_power: f64,
    option_buying_power: f64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountTypes {
    SecuritiesAccount(SecuritiesAccount),
    CashAccount(CashAccount),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    account_number: String,
    round_trips: i32,
    is_day_trader: bool,
    is_closing_only_restricted: bool,
    pfcb_flag: bool,
    positions: Vec<Position>,
    initial_balances: MarginInitialBalance,
    current_balances: MarginBalance,
    projected_balances: MarginBalance,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {}

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
pub struct UserDetails {
cd_domain_id	: String,
login	: String,
r#type	: UserDetailsType,
user_id : i64,
system_user_name	: String,
first_name	: String,
last_name	: String,
broker_rep_code	: String,
}

#[derive(Deserialize, Debug)]
pub enum UserDetailsType {
#[serde(rename(deserialize = "ADVISOR_USER"))]
    AdvisorUser,
#[serde(rename(deserialize = "BROKER_USER"))]
BrokerUser, 
#[serde(rename(deserialize = "CLIENT_USER"))]
ClientUser, 
#[serde(rename(deserialize = "SYSTEM_USER"))]
SystemUser, 
#[serde(rename(deserialize = "UNKNOWN"))]
Unknown
}

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
pub struct TransferItem {
    instrument: TransactionInstrument, 
    amount: f64, 
    cost: f64, 
    price: f64, 
    fee_type: FeeType, 
    position_effect: PositionEffect, 
}


#[derive(Deserialize, Debug)]
pub enum FeeType{
#[serde(rename(deserialize = "COMMISSION"))]
    Commission,
#[serde(rename(deserialize = "SEC_FEE"))]
    SecFee,
#[serde(rename(deserialize = "STR_FEE"))]
    StrFee,
#[serde(rename(deserialize = "R_FEE"))]
    RFee,
#[serde(rename(deserialize = "CDSC_FEE"))]
    CdcscFee,
#[serde(rename(deserialize = "OPT_REG_FEE"))]
    OptRegFee,
#[serde(rename(deserialize = "ADDITIONAL_FEE"))]
    AdditionalFee,
#[serde(rename(deserialize = "MISCELLANEOUS_FEE"))]
    MiscellaneousFee,
#[serde(rename(deserialize = "FUTURES_EXCHANGE_FEE"))]
    FuturesExchangeFee, 
#[serde(rename(deserialize = "LOW_PROCEEDS_COMMISSION"))]
    LowProceedsCommission, 
#[serde(rename(deserialize = "BASE_CHARGE"))]
    BaseCharge,
#[serde(rename(deserialize = "GENERAL_CHARGE"))]
    GeneralCharge, 
#[serde(rename(deserialize = "GST_FEE"))]
    GstFee, 
#[serde(rename(deserialize = "TAF_FEE"))]
    TafFee, 
#[serde(rename(deserialize = "INDEX_OPTION_FEE"))]
    IndexOptionFee, 
#[serde(rename(deserialize = "UNKNOWN"))]
    Unknown
}


#[derive(Deserialize, Debug)]
pub enum PositionEffect{
#[serde(rename(deserialize = "OPENING"))]
Opening, 
#[serde(rename(deserialize = "CLOSING"))]
Closing, 
#[serde(rename(deserialize = "AUTOMATIC"))]
Automatic, 
#[serde(rename(deserialize = "UNKNOWN"))]
Unknown

}



#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionInstrument {
/*
	TransactionInstrument{
oneOf ->	
TransactionCashEquivalent{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ SWEEP_VEHICLE, SAVINGS, MONEY_MARKET_FUND, UNKNOWN ]
}
CollectiveInvestment{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ UNIT_INVESTMENT_TRUST, EXCHANGE_TRADED_FUND, CLOSED_END_FUND, INDEX, UNITS ]
}
Currency{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
}
TransactionEquity{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ COMMON_STOCK, PREFERRED_STOCK, DEPOSITORY_RECEIPT, PREFERRED_DEPOSITORY_RECEIPT, RESTRICTED_STOCK, COMPONENT_UNIT, RIGHT, WARRANT, CONVERTIBLE_PREFERRED_STOCK, CONVERTIBLE_STOCK, LIMITED_PARTNERSHIP, WHEN_ISSUED, UNKNOWN ]
}
TransactionFixedIncome{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ BOND_UNIT, CERTIFICATE_OF_DEPOSIT, CONVERTIBLE_BOND, COLLATERALIZED_MORTGAGE_OBLIGATION, CORPORATE_BOND, GOVERNMENT_MORTGAGE, GNMA_BONDS, MUNICIPAL_ASSESSMENT_DISTRICT, MUNICIPAL_BOND, OTHER_GOVERNMENT, SHORT_TERM_PAPER, US_TREASURY_BOND, US_TREASURY_BILL, US_TREASURY_NOTE, US_TREASURY_ZERO_COUPON, AGENCY_BOND, WHEN_AS_AND_IF_ISSUED_BOND, ASSET_BACKED_SECURITY, UNKNOWN ]
maturityDate	string($date-time)
factor	number($double)
multiplier	number($double)
variableRate	number($double)
}
Forex{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ STANDARD, NBBO, UNKNOWN ]
baseCurrency	Currency{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
}
counterCurrency	Currency{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
}
}
Future{
activeContract	boolean
default: false
type	string
Enum:
[ STANDARD, UNKNOWN ]
expirationDate	string($date-time)
lastTradingDate	string($date-time)
firstNoticeDate	string($date-time)
multiplier	number($double)
}
Index{
activeContract	boolean
default: false
type	string
Enum:
[ BROAD_BASED, NARROW_BASED, UNKNOWN ]
}
TransactionMutualFund{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
fundFamilyName	string
fundFamilySymbol	string
fundGroup	string
type	string
Enum:
[ NOT_APPLICABLE, OPEN_END_NON_TAXABLE, OPEN_END_TAXABLE, NO_LOAD_NON_TAXABLE, NO_LOAD_TAXABLE, UNKNOWN ]
exchangeCutoffTime	string($date-time)
purchaseCutoffTime	string($date-time)
redemptionCutoffTime	string($date-time)
}
TransactionOption{
assetType*	string
Enum:
Array [ 8 ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
expirationDate	string($date-time)
optionDeliverables	[
xml: OrderedMap { "name": "optionDeliverables", "wrapped": true }
TransactionAPIOptionDeliverable{
rootSymbol	string
strikePercent	integer($int64)
deliverableNumber	integer($int64)
deliverableUnits	number($double)
deliverable	{
}
assetType	assetTypestring
Enum:
[ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ]
}]
optionPremiumMultiplier	integer($int64)
putCall	string
Enum:
[ PUT, CALL, UNKNOWN ]
strikePrice	number($double)
type	string
Enum:
[ VANILLA, BINARY, BARRIER, UNKNOWN ]
underlyingSymbol	string
underlyingCusip	string
deliverable	{
}
}
Product{
assetType*	string
Enum:
[ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
cusip	string
symbol	string
description	string
instrumentId	integer($int64)
netChange	number($double)
type	string
Enum:
[ TBD, UNKNOWN ]
}
}
*/
}

use serde::{Deserialize, Serialize};
use std::fmt;

pub type AccountNumbers = Vec<AccountNumber>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountNumber {
    pub account_number: String,
    pub hash_value: String,
}

pub type Accounts = Vec<AccountTypes>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountTypes {
    pub securities_account: Option<SecuritiesAccount>,
    pub cash_account: Option<CashAccount>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "assetType")] // why is this here? 
pub enum AccountInstrument {
    #[serde(rename(serialize = "COLLECTIVE_INVESTMENT"))]
    #[serde(rename(deserialize = "COLLECTIVE_INVESTMENT"))]
    CollectiveInvestment {
        cusip: String,
        symbol: String,
        description: String,
        r#type: String,
    },

    #[serde(rename(serialize = "OPTION"))]
    #[serde(rename(deserialize = "OPTION"))]
    Option {
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
        r#type: String,

        #[serde(rename(serialize = "putCall"))]
        #[serde(rename(deserialize = "putCall"))]
        put_call: String,

        #[serde(rename(serialize = "underlyingSymbol"))]
        #[serde(rename(deserialize = "underlyingSymbol"))]
        underlying_symbol: String,
    },

    #[serde(rename(serialize = "EQUITY"))]
    #[serde(rename(deserialize = "EQUITY"))]
    Equity {
        cusip: String,
        symbol: String,

        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
    },
}

impl AccountInstrument {
    pub fn symbol(&self) -> String {
        (match self {
            Self::CollectiveInvestment { symbol, .. } => symbol,
            Self::Option { symbol, .. } => symbol,
            Self::Equity { symbol, .. } => symbol,
        })
        .clone()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: f64,
    pub average_price: f64,
    pub current_day_profit_loss: f64,
    pub current_day_profit_loss_percentage: f64,
    pub long_quantity: f64,
    pub settled_long_quantity: f64,
    pub settled_short_quantity: f64,
    pub aged_quantity: Option<f64>,
    pub instrument: AccountInstrument,
    pub market_value: f64,
    pub maintenance_requirement: Option<f64>,
    pub average_long_price: Option<f64>,
    pub average_short_price: Option<f64>,
    pub tax_log_average_price: Option<f64>,
    pub tax_log_average_short_price: Option<f64>,
    pub long_open_profit_loss: f64,
    pub short_open_profit_loss: Option<f64>,
    pub previous_session_long_quantity: Option<f64>,
    pub previous_session_short_quantity: Option<f64>,
    pub current_day_cost: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginInitialBalance {
    pub accrued_interest: f64,
    pub available_funds_non_marginable_trade: f64,
    pub bond_value: f64,
    pub buying_power: f64,
    pub cash_balance: f64,
    pub cash_available_for_trading: f64,
    pub cash_receipts: f64,
    pub day_trading_buying_power: Option<f64>,
    pub day_trading_buying_power_call: Option<f64>,
    pub day_trading_equity_call: Option<f64>,
    pub equity: Option<f64>,
    pub equity_percentage: Option<f64>,
    pub liquidation_value: f64,
    pub long_margin_value: Option<f64>,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    pub maintenance_call: f64,
    pub maintenance_requirement: Option<f64>,
    pub margin: f64,
    pub margin_equity: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub reg_t_call: f64,
    pub short_margin_value: Option<f64>,
    pub short_option_market_value: f64,
    pub short_stick_value: Option<f64>,
    pub total_cash: f64,
    pub is_in_call: bool,
    pub unsettled_cash: Option<f64>,
    pub pending_deposits: f64,
    pub margin_balance: f64,
    pub short_balance: Option<f64>,
    pub account_value: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalance {
    pub available_funds: f64,
    pub available_funds_non_marginable_trade: f64,
    pub buying_power: f64,
    pub buying_power_non_marginable_trade: Option<f64>,
    pub day_trading_buying_power: Option<f64>,
    pub day_trading_buying_power_call: Option<f64>,
    pub equity: Option<f64>,
    pub equity_percentage: Option<f64>,
    pub long_margin_value: Option<f64>,
    pub maintenance_call: f64,
    pub maintenance_requirement: Option<f64>,
    pub margin_balance: Option<f64>,
    pub reg_t_call: f64,
    pub short_balance: Option<f64>,
    pub short_margin_value: Option<f64>,
    pub sma: Option<f64>,
    pub is_in_call: Option<bool>,
    pub stock_buying_power: Option<f64>,
    pub option_buying_power: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    pub account_number: String,
    pub round_trips: i32,
    pub is_day_trader: bool,
    pub is_closing_only_restricted: bool,
    pub pfcb_flag: bool,
    pub positions: Vec<Position>,
    pub initial_balances: MarginInitialBalance,
    pub current_balances: MarginBalance,
    pub projected_balances: MarginBalance,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {}

pub type Transactions = Vec<Transaction>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub activity_id: i64,
    pub time: String,
    pub user: Option<UserDetails>,
    pub description: Option<String>,
    pub account_number: String,
    pub r#type: TransactionType,
    pub status: TransactionStatus,
    pub sub_account: SubAccount,
    pub trade_date: String,
    pub settlement_date: Option<String>,
    pub position_id: i64,
    pub order_id: Option<i64>,
    pub net_amount: f64,
    pub activity_type: Option<TransactionActivityType>,
    pub transfer_items: Vec<TransferItem>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub cd_domain_id: String,
    pub login: String,
    pub r#type: UserDetailsType,
    pub user_id: i64,
    pub system_user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub broker_rep_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserDetailsType {
    #[serde(rename(serialize = "ADVISOR_USER"))]
    #[serde(rename(deserialize = "ADVISOR_USER"))]
    AdvisorUser,
    #[serde(rename(serialize = "BROKER_USER"))]
    #[serde(rename(deserialize = "BROKER_USER"))]
    BrokerUser,
    #[serde(rename(serialize = "CLIENT_USER"))]
    #[serde(rename(deserialize = "CLIENT_USER"))]
    ClientUser,
    #[serde(rename(serialize = "SYSTEM_USER"))]
    #[serde(rename(deserialize = "SYSTEM_USER"))]
    SystemUser,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionType {
    #[serde(rename(serialize = "TRADE"))]
    #[serde(rename(deserialize = "TRADE"))]
    Trade,

    #[serde(rename(serialize = "RECEIVE_AND_DELIVER"))]
    #[serde(rename(deserialize = "RECEIVE_AND_DELIVER"))]
    ReceiveAndDeliver,

    #[serde(rename(serialize = "DIVIDEND_OR_INTEREST"))]
    #[serde(rename(deserialize = "DIVIDEND_OR_INTEREST"))]
    DividendOrInterest,

    #[serde(rename(serialize = "ACH_RECEIPT"))]
    #[serde(rename(deserialize = "ACH_RECEIPT"))]
    AchReceipt,

    #[serde(rename(serialize = "ACH_DISBURSEMENT"))]
    #[serde(rename(deserialize = "ACH_DISBURSEMENT"))]
    AchDisbursement,

    #[serde(rename(serialize = "CASH_RECEIPT"))]
    #[serde(rename(deserialize = "CASH_RECEIPT"))]
    CashReceipt,

    #[serde(rename(serialize = "CASH_DISBURSEMENT"))]
    #[serde(rename(deserialize = "CASH_DISBURSEMENT"))]
    CashDisbursement,

    #[serde(rename(serialize = "ELECTRONIC_FUND"))]
    #[serde(rename(deserialize = "ELECTRONIC_FUND"))]
    ElectronicFund,

    #[serde(rename(serialize = "WIRE_OUT"))]
    #[serde(rename(deserialize = "WIRE_OUT"))]
    WireOut,

    #[serde(rename(serialize = "WIRE_IN"))]
    #[serde(rename(deserialize = "WIRE_IN"))]
    WireIn,

    #[serde(rename(serialize = "JOURNAL"))]
    #[serde(rename(deserialize = "JOURNAL"))]
    Journal,

    #[serde(rename(serialize = "MEMORANDUM"))]
    #[serde(rename(deserialize = "MEMORANDUM"))]
    Memorandum,

    #[serde(rename(serialize = "MARGIN_CALL"))]
    #[serde(rename(deserialize = "MARGIN_CALL"))]
    MarginCall,

    #[serde(rename(serialize = "MONEY_MARKET"))]
    #[serde(rename(deserialize = "MONEY_MARKET"))]
    MoneyMarket,

    #[serde(rename(serialize = "SMA_ADJUSTMENT"))]
    #[serde(rename(deserialize = "SMA_ADJUSTMENT"))]
    SmaAdjustment,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransactionType::Trade => "TRADE",
                TransactionType::ReceiveAndDeliver => "RECEIVE_AND_DELIVER",
                TransactionType::DividendOrInterest => "DIVIDEND_OR_INTEREST",
                TransactionType::AchReceipt => "ACH_RECEIPT",
                TransactionType::AchDisbursement => "ACH_DISBURSEMENT",
                TransactionType::CashReceipt => "CASH_RECEIPT",
                TransactionType::CashDisbursement => "CASH_DISBURSEMENT",
                TransactionType::ElectronicFund => "ELECTRONIC_FUND",
                TransactionType::WireOut => "WIRE_OUT",
                TransactionType::WireIn => "WIRE_IN",
                TransactionType::Journal => "JOURNAL",
                TransactionType::Memorandum => "MEMORANDUM",
                TransactionType::MarginCall => "MARGIN_CALL",
                TransactionType::MoneyMarket => "MONEY_MARKET",
                TransactionType::SmaAdjustment => "SMA_ADJUSTMENT",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionStatus {
    #[serde(rename(serialize = "VALID"))]
    #[serde(rename(deserialize = "VALID"))]
    Valid,

    #[serde(rename(serialize = "INVALID"))]
    #[serde(rename(deserialize = "INVALID"))]
    Invalid,

    #[serde(rename(serialize = "PENDING"))]
    #[serde(rename(deserialize = "PENDING"))]
    Pending,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SubAccount {
    #[serde(rename(serialize = "CASH"))]
    #[serde(rename(deserialize = "CASH"))]
    Cash,

    #[serde(rename(serialize = "MARGIN"))]
    #[serde(rename(deserialize = "MARGIN"))]
    Margin,

    #[serde(rename(serialize = "SHORT"))]
    #[serde(rename(deserialize = "SHORT"))]
    Short,

    #[serde(rename(serialize = "DIV"))]
    #[serde(rename(deserialize = "DIV"))]
    Div,

    #[serde(rename(serialize = "INCOME"))]
    #[serde(rename(deserialize = "INCOME"))]
    Income,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionActivityType {
    #[serde(rename(serialize = "ACTIVITY_CORRECTION"))]
    #[serde(rename(deserialize = "ACTIVITY_CORRECTION"))]
    ActivityCorrection,

    #[serde(rename(serialize = "EXECUTION"))]
    #[serde(rename(deserialize = "EXECUTION"))]
    Execution,

    #[serde(rename(serialize = "ORDER_ACTION"))]
    #[serde(rename(deserialize = "ORDER_ACTION"))]
    OrderAction,

    #[serde(rename(serialize = "TRANSFER"))]
    #[serde(rename(deserialize = "TRANSFER"))]
    Transfer,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    pub instrument: TransactionInstrument,
    pub amount: f64,
    pub cost: f64,
    pub price: Option<f64>,
    pub fee_type: Option<FeeType>,
    pub position_effect: Option<PositionEffect>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum FeeType {
    #[serde(rename(serialize = "COMMISSION"))]
    #[serde(rename(deserialize = "COMMISSION"))]
    Commission,
    #[serde(rename(serialize = "SEC_FEE"))]
    #[serde(rename(deserialize = "SEC_FEE"))]
    SecFee,
    #[serde(rename(serialize = "STR_FEE"))]
    #[serde(rename(deserialize = "STR_FEE"))]
    StrFee,
    #[serde(rename(serialize = "R_FEE"))]
    #[serde(rename(deserialize = "R_FEE"))]
    RFee,
    #[serde(rename(serialize = "CDSC_FEE"))]
    #[serde(rename(deserialize = "CDSC_FEE"))]
    CdcscFee,
    #[serde(rename(serialize = "OPT_REG_FEE"))]
    #[serde(rename(deserialize = "OPT_REG_FEE"))]
    OptRegFee,
    #[serde(rename(serialize = "ADDITIONAL_FEE"))]
    #[serde(rename(deserialize = "ADDITIONAL_FEE"))]
    AdditionalFee,
    #[serde(rename(serialize = "MISCELLANEOUS_FEE"))]
    #[serde(rename(deserialize = "MISCELLANEOUS_FEE"))]
    MiscellaneousFee,
    #[serde(rename(serialize = "FUTURES_EXCHANGE_FEE"))]
    #[serde(rename(deserialize = "FUTURES_EXCHANGE_FEE"))]
    FuturesExchangeFee,
    #[serde(rename(serialize = "LOW_PROCEEDS_COMMISSION"))]
    #[serde(rename(deserialize = "LOW_PROCEEDS_COMMISSION"))]
    LowProceedsCommission,
    #[serde(rename(serialize = "BASE_CHARGE"))]
    #[serde(rename(deserialize = "BASE_CHARGE"))]
    BaseCharge,
    #[serde(rename(serialize = "GENERAL_CHARGE"))]
    #[serde(rename(deserialize = "GENERAL_CHARGE"))]
    GeneralCharge,
    #[serde(rename(serialize = "GST_FEE"))]
    #[serde(rename(deserialize = "GST_FEE"))]
    GstFee,
    #[serde(rename(serialize = "TAF_FEE"))]
    #[serde(rename(deserialize = "TAF_FEE"))]
    TafFee,
    #[serde(rename(serialize = "INDEX_OPTION_FEE"))]
    #[serde(rename(deserialize = "INDEX_OPTION_FEE"))]
    IndexOptionFee,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PositionEffect {
    #[serde(rename(serialize = "OPENING"))]
    #[serde(rename(deserialize = "OPENING"))]
    Opening,
    #[serde(rename(serialize = "CLOSING"))]
    #[serde(rename(deserialize = "CLOSING"))]
    Closing,
    #[serde(rename(serialize = "AUTOMATIC"))]
    #[serde(rename(deserialize = "AUTOMATIC"))]
    Automatic,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AssetType {
    #[serde(rename(serialize = "EQUITY"))]
    #[serde(rename(deserialize = "EQUITY"))]
    Equity,

    #[serde(rename(serialize = "OPTION"))]
    #[serde(rename(deserialize = "OPTION"))]
    Option,

    #[serde(rename(serialize = "INDEX"))]
    #[serde(rename(deserialize = "INDEX"))]
    Index,

    #[serde(rename(serialize = "MUTUAL_FUND"))]
    #[serde(rename(deserialize = "MUTUAL_FUND"))]
    MutualFund,

    #[serde(rename(serialize = "CASH_EQUIVALENT"))]
    #[serde(rename(deserialize = "CASH_EQUIVALENT"))]
    CashEquivalent,

    #[serde(rename(serialize = "FIXED_INCOME"))]
    #[serde(rename(deserialize = "FIXED_INCOME"))]
    FixedIncome,

    #[serde(rename(serialize = "CURRENCY"))]
    #[serde(rename(deserialize = "CURRENCY"))]
    Currency,

    #[serde(rename(serialize = "COLLECTIVE_INVESTMENT"))]
    #[serde(rename(deserialize = "COLLECTIVE_INVESTMENT"))]
    CollectiveInvestment,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionCashEquivalentType {
    #[serde(rename(serialize = "SWEEP_VEHICLE"))]
    #[serde(rename(deserialize = "SWEEP_VEHICLE"))]
    SweepVehicle,
    #[serde(rename(serialize = "SAVINGS"))]
    #[serde(rename(deserialize = "SAVINGS"))]
    Savings,
    #[serde(rename(serialize = "MONEY_MARKET_FUND"))]
    #[serde(rename(deserialize = "MONEY_MARKET_FUND"))]
    MoneyMarketFund,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CollectiveInvestmentType {
    #[serde(rename(serialize = "UNIT_INVESTMENT_TRUST"))]
    #[serde(rename(deserialize = "UNIT_INVESTMENT_TRUST"))]
    UnitInvestmentTrust,
    #[serde(rename(serialize = "EXCHANGE_TRADED_FUND"))]
    #[serde(rename(deserialize = "EXCHANGE_TRADED_FUND"))]
    ExchangeTradedFund,
    #[serde(rename(serialize = "CLOSED_END_FUND"))]
    #[serde(rename(deserialize = "CLOSED_END_FUND"))]
    ClosedEndFund,
    #[serde(rename(serialize = "INDEX"))]
    #[serde(rename(deserialize = "INDEX"))]
    Index,
    #[serde(rename(serialize = "UNITS"))]
    #[serde(rename(deserialize = "UNITS"))]
    Units,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionEquityType {
    #[serde(rename(serialize = "COMMON_STOCK"))]
    #[serde(rename(deserialize = "COMMON_STOCK"))]
    CommonStock,
    #[serde(rename(serialize = "PREFERRED_STOCK"))]
    #[serde(rename(deserialize = "PREFERRED_STOCK"))]
    PreferredStock,
    #[serde(rename(serialize = "DEPOSITORY_RECEIPT"))]
    #[serde(rename(deserialize = "DEPOSITORY_RECEIPT"))]
    DepositoryReceipt,
    #[serde(rename(serialize = "PREFERRED_DEPOSITORY_RECEIPT"))]
    #[serde(rename(deserialize = "PREFERRED_DEPOSITORY_RECEIPT"))]
    PreferredDepositoryReceipt,
    #[serde(rename(serialize = "RESTRICTED_STOCK"))]
    #[serde(rename(deserialize = "RESTRICTED_STOCK"))]
    RestrictedStock,
    #[serde(rename(serialize = "COMPONENT_UNIT"))]
    #[serde(rename(deserialize = "COMPONENT_UNIT"))]
    ComponentUnit,
    #[serde(rename(serialize = "RIGHT"))]
    #[serde(rename(deserialize = "RIGHT"))]
    Right,
    #[serde(rename(serialize = "WARRANT"))]
    #[serde(rename(deserialize = "WARRANT"))]
    Warrant,
    #[serde(rename(serialize = "CONVERTIBLE_PREFERRED_STOCK"))]
    #[serde(rename(deserialize = "CONVERTIBLE_PREFERRED_STOCK"))]
    ConvertiblePreferredStock,
    #[serde(rename(serialize = "CONVERTIBLE_STOCK"))]
    #[serde(rename(deserialize = "CONVERTIBLE_STOCK"))]
    ConvertibleStock,
    #[serde(rename(serialize = "LIMITED_PARTNERSHIP"))]
    #[serde(rename(deserialize = "LIMITED_PARTNERSHIP"))]
    LimitedPartnership,
    #[serde(rename(serialize = "WHEN_ISSUED"))]
    #[serde(rename(deserialize = "WHEN_ISSUED"))]
    WhenIssued,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionFixedIncomeType {
    #[serde(rename(serialize = "BOND_UNIT"))]
    #[serde(rename(deserialize = "BOND_UNIT"))]
    BondUnit,

    #[serde(rename(serialize = "CERTIFICATE_OF_DEPOSIT"))]
    #[serde(rename(deserialize = "CERTIFICATE_OF_DEPOSIT"))]
    CertificateOfDeposit,

    #[serde(rename(serialize = "CONVERTIBLE_BOND"))]
    #[serde(rename(deserialize = "CONVERTIBLE_BOND"))]
    ConvertibleBond,

    #[serde(rename(serialize = "COLLATERALIZED_MORTGAGE_OBLIGATION"))]
    #[serde(rename(deserialize = "COLLATERALIZED_MORTGAGE_OBLIGATION"))]
    CollateralizedMortgageObligation,

    #[serde(rename(serialize = "CORPORATE_BOND"))]
    #[serde(rename(deserialize = "CORPORATE_BOND"))]
    CorporateBond,

    #[serde(rename(serialize = "GOVERNMENT_MORTGAGE"))]
    #[serde(rename(deserialize = "GOVERNMENT_MORTGAGE"))]
    GovernmentMortgage,

    #[serde(rename(serialize = "GNMA_BONDS"))]
    #[serde(rename(deserialize = "GNMA_BONDS"))]
    GnmaBonds,

    #[serde(rename(serialize = "MUNICIPAL_ASSESSMENT_DISTRICT"))]
    #[serde(rename(deserialize = "MUNICIPAL_ASSESSMENT_DISTRICT"))]
    MunicipalAssessmentDistrict,

    #[serde(rename(serialize = "MUNICIPAL_BOND"))]
    #[serde(rename(deserialize = "MUNICIPAL_BOND"))]
    MunicipalBond,

    #[serde(rename(serialize = "OTHER_GOVERNMENT"))]
    #[serde(rename(deserialize = "OTHER_GOVERNMENT"))]
    OtherGovernment,

    #[serde(rename(serialize = "SHORT_TERM_PAPER"))]
    #[serde(rename(deserialize = "SHORT_TERM_PAPER"))]
    ShortTermPaper,

    #[serde(rename(serialize = "US_TREASURY_BOND"))]
    #[serde(rename(deserialize = "US_TREASURY_BOND"))]
    UsTreasuryBond,

    #[serde(rename(serialize = "US_TREASURY_BILL"))]
    #[serde(rename(deserialize = "US_TREASURY_BILL"))]
    UsTreasuryBill,

    #[serde(rename(serialize = "US_TREASURY_NOTE"))]
    #[serde(rename(deserialize = "US_TREASURY_NOTE"))]
    UsTreasuryNote,

    #[serde(rename(serialize = "US_TREASURY_ZERO_COUPON"))]
    #[serde(rename(deserialize = "US_TREASURY_ZERO_COUPON"))]
    UsTreasuryZeroCoupon,

    #[serde(rename(serialize = "AGENCY_BOND"))]
    #[serde(rename(deserialize = "AGENCY_BOND"))]
    AgencyBond,

    #[serde(rename(serialize = "WHEN_AS_AND_IF_ISSUED_BOND"))]
    #[serde(rename(deserialize = "WHEN_AS_AND_IF_ISSUED_BOND"))]
    WhenAsAndIfissuedBond,

    #[serde(rename(serialize = "ASSET_BACKED_SECURITY"))]
    #[serde(rename(deserialize = "ASSET_BACKED_SECURITY"))]
    AssetBackedSecurity,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ForexType {
    #[serde(rename(serialize = "STANDARD"))]
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,
    #[serde(rename(serialize = "NBBO"))]
    #[serde(rename(deserialize = "NBBO"))]
    Nbbo,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum FutureType {
    #[serde(rename(serialize = "STANDARD"))]
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum IndexType {
    #[serde(rename(serialize = "BROAD_BASED"))]
    #[serde(rename(deserialize = "BROAD_BASED"))]
    BroadBased,
    #[serde(rename(serialize = "NARROW_BASED"))]
    #[serde(rename(deserialize = "NARROW_BASED"))]
    NarrowBased,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionMutualFundType {
    #[serde(rename(serialize = "NOT_APPLICABLE"))]
    #[serde(rename(deserialize = "NOT_APPLICABLE"))]
    NotApplicable,
    #[serde(rename(serialize = "OPEN_END_NON_TAXABLE"))]
    #[serde(rename(deserialize = "OPEN_END_NON_TAXABLE"))]
    OpenEndNonTaxable,
    #[serde(rename(serialize = "OPEN_END_TAXABLE"))]
    #[serde(rename(deserialize = "OPEN_END_TAXABLE"))]
    OpenEndTaxable,
    #[serde(rename(serialize = "NO_LOAD_NON_TAXABLE"))]
    #[serde(rename(deserialize = "NO_LOAD_NON_TAXABLE"))]
    NoLoadNonTaxable,
    #[serde(rename(serialize = "NO_LOAD_TAXABLE"))]
    #[serde(rename(deserialize = "NO_LOAD_TAXABLE"))]
    NoLoadTaxable,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PutCallType {
    #[serde(rename(serialize = "PUT"))]
    #[serde(rename(deserialize = "PUT"))]
    Put,
    #[serde(rename(serialize = "CALL"))]
    #[serde(rename(deserialize = "CALL"))]
    Call,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TransactionOptionType {
    #[serde(rename(serialize = "VANILLA"))]
    #[serde(rename(deserialize = "VANILLA"))]
    Vanilla,
    #[serde(rename(serialize = "BINARY"))]
    #[serde(rename(deserialize = "BINARY"))]
    Binary,
    #[serde(rename(serialize = "BARRIER"))]
    #[serde(rename(deserialize = "BARRIER"))]
    Barrier,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ProductType {
    #[serde(rename(serialize = "TBD"))]
    #[serde(rename(deserialize = "TBD"))]
    Tbd,
    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    // deliverable	Some empty type?
    pub deliverable: TransactionInstrument,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectiveInvestment {
    pub asset_type: AssetType,
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: f64,
    pub r#type: CollectiveInvestmentType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum TransactionInstrument {
    TransactionEquity {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        symbol: String,
        status: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "closingPrice"))]
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        te_type: TransactionEquityType,
    },

    CollectiveInvestment {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        status: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "closingPrice"))]
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        ci_type: CollectiveInvestmentType,
    },

    TransactionOption {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        status: String,
        symbol: String,
        #[serde(rename(serialize = "closingPrice"))]
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "expirationDate"))]
        #[serde(rename(deserialize = "expirationDate"))]
        expiration_date: String,
        #[serde(rename(serialize = "optionDeliverables"))]
        #[serde(rename(deserialize = "optionDeliverables"))]
        option_deliverables: Vec<TransactionAPIOptionDeliverable>,
        #[serde(rename(serialize = "optionPremiumMultiplier"))]
        #[serde(rename(deserialize = "optionPremiumMultiplier"))]
        option_premium_multiplier: i64,
        #[serde(rename(serialize = "putCall"))]
        #[serde(rename(deserialize = "putCall"))]
        put_call: PutCallType,
        #[serde(rename(serialize = "strikePrice"))]
        #[serde(rename(deserialize = "strikePrice"))]
        strike_price: f64,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        to_type: TransactionOptionType,
        #[serde(rename(serialize = "underlyingSymbol"))]
        #[serde(rename(deserialize = "underlyingSymbol"))]
        underlying_symbol: String,
        #[serde(rename(serialize = "underlyingCusip"))]
        #[serde(rename(deserialize = "underlyingCusip"))]
        underlying_cusip: String,
    },

    Currency {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: Option<f64>,
        status: String,
        #[serde(rename(serialize = "closingPrice"))]
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
    },
    /*
    Other types I may implement in the future

    TransactionCashEquivalent { },

    TransactionFixedIncome { },

    Forex { },

    Future { },

    Index { },

    TransactionMutualFund { },

    Product { },
    */
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderSession {
    #[serde(rename(serialize = "NORMAL"))]
    #[serde(rename(deserialize = "NORMAL"))]
    Normal,

    #[serde(rename(serialize = "AM"))]
    #[serde(rename(deserialize = "AM"))]
    Am,

    #[serde(rename(serialize = "PM"))]
    #[serde(rename(deserialize = "PM"))]
    Pm,

    #[serde(rename(serialize = "SEAMLESS"))]
    #[serde(rename(deserialize = "SEAMLESS"))]
    Seamless,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderTypeRequest {
    #[serde(rename(serialize = "MARKET"))]
    #[serde(rename(deserialize = "MARKET"))]
    Market,

    #[serde(rename(serialize = "LIMIT"))]
    #[serde(rename(deserialize = "LIMIT"))]
    Limit,

    #[serde(rename(serialize = "STOP"))]
    #[serde(rename(deserialize = "STOP"))]
    Stop,

    #[serde(rename(serialize = "STOP_LIMIT"))]
    #[serde(rename(deserialize = "STOP_LIMIT"))]
    StopLimit,

    #[serde(rename(serialize = "TRAILING_STOP"))]
    #[serde(rename(deserialize = "TRAILING_STOP"))]
    TrailingStop,

    #[serde(rename(serialize = "CABINET"))]
    #[serde(rename(deserialize = "CABINET"))]
    Cabinet,

    #[serde(rename(serialize = "NON_MARKETABLE"))]
    #[serde(rename(deserialize = "NON_MARKETABLE"))]
    NonMarketable,

    #[serde(rename(serialize = "MARKET_ON_CLOSE"))]
    #[serde(rename(deserialize = "MARKET_ON_CLOSE"))]
    MarketOnClose,

    #[serde(rename(serialize = "EXERCISE"))]
    #[serde(rename(deserialize = "EXERCISE"))]
    Exercise,

    #[serde(rename(serialize = "TRAILING_STOP_LIMIT"))]
    #[serde(rename(deserialize = "TRAILING_STOP_LIMIT"))]
    TrailingStopLimit,

    #[serde(rename(serialize = "NET_DEBIT"))]
    #[serde(rename(deserialize = "NET_DEBIT"))]
    NetDebit,

    #[serde(rename(serialize = "NET_CREDIT"))]
    #[serde(rename(deserialize = "NET_CREDIT"))]
    NetCredit,

    #[serde(rename(serialize = "NET_ZERO"))]
    #[serde(rename(deserialize = "NET_ZERO"))]
    NetZero,

    #[serde(rename(serialize = "LIMIT_ON_CLOSE"))]
    #[serde(rename(deserialize = "LIMIT_ON_CLOSE"))]
    LimitOnClose,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ComplexOrderStrategyType {
    #[serde(rename(serialize = "NONE"))]
    #[serde(rename(deserialize = "NONE"))]
    None,

    #[serde(rename(serialize = "COVERED"))]
    #[serde(rename(deserialize = "COVERED"))]
    Covered,

    #[serde(rename(serialize = "VERTICAL"))]
    #[serde(rename(deserialize = "VERTICAL"))]
    Vertical,

    #[serde(rename(serialize = "BACK_RATIO"))]
    #[serde(rename(deserialize = "BACK_RATIO"))]
    BackRatio,

    #[serde(rename(serialize = "CALENDAR"))]
    #[serde(rename(deserialize = "CALENDAR"))]
    Calendar,

    #[serde(rename(serialize = "DIAGONAL"))]
    #[serde(rename(deserialize = "DIAGONAL"))]
    Diagonal,

    #[serde(rename(serialize = "STRADDLE"))]
    #[serde(rename(deserialize = "STRADDLE"))]
    Straddle,

    #[serde(rename(serialize = "STRANGLE"))]
    #[serde(rename(deserialize = "STRANGLE"))]
    Strangle,

    #[serde(rename(serialize = "COLLAR_SYNTHETIC"))]
    #[serde(rename(deserialize = "COLLAR_SYNTHETIC"))]
    CollarSynthetic,

    #[serde(rename(serialize = "BUTTERFLY"))]
    #[serde(rename(deserialize = "BUTTERFLY"))]
    Butterfly,

    #[serde(rename(serialize = "CONDOR"))]
    #[serde(rename(deserialize = "CONDOR"))]
    Condor,

    #[serde(rename(serialize = "IRON_CONDOR"))]
    #[serde(rename(deserialize = "IRON_CONDOR"))]
    IronCondor,

    #[serde(rename(serialize = "VERTICAL_ROLL"))]
    #[serde(rename(deserialize = "VERTICAL_ROLL"))]
    VerticalRoll,

    #[serde(rename(serialize = "COLLAR_WITH_STOCK"))]
    #[serde(rename(deserialize = "COLLAR_WITH_STOCK"))]
    CollarWithStock,

    #[serde(rename(serialize = "DOUBLE_DIAGONAL"))]
    #[serde(rename(deserialize = "DOUBLE_DIAGONAL"))]
    DoubleDiagonal,

    #[serde(rename(serialize = "UNBALANCED_BUTTERFLY"))]
    #[serde(rename(deserialize = "UNBALANCED_BUTTERFLY"))]
    UnbalancedButterfly,

    #[serde(rename(serialize = "UNBALANCED_CONDOR"))]
    #[serde(rename(deserialize = "UNBALANCED_CONDOR"))]
    UnbalancedCondor,

    #[serde(rename(serialize = "UNBALANCED_IRON_CONDOR"))]
    #[serde(rename(deserialize = "UNBALANCED_IRON_CONDOR"))]
    UnbalancedIronCondor,

    #[serde(rename(serialize = "UNBALANCED_VERTICAL_ROLL"))]
    #[serde(rename(deserialize = "UNBALANCED_VERTICAL_ROLL"))]
    UnbalancedVerticalRoll,

    #[serde(rename(serialize = "MUTUAL_FUND_SWAP"))]
    #[serde(rename(deserialize = "MUTUAL_FUND_SWAP"))]
    MutualFundSwap,

    #[serde(rename(serialize = "CUSTOM"))]
    #[serde(rename(deserialize = "CUSTOM"))]
    Custom,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StopPriceLinkBasis {
    #[serde(rename(serialize = "MANUAL"))]
    #[serde(rename(deserialize = "MANUAL"))]
    Manual,

    #[serde(rename(serialize = "BASE"))]
    #[serde(rename(deserialize = "BASE"))]
    Base,

    #[serde(rename(serialize = "TRIGGER"))]
    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,

    #[serde(rename(serialize = "LAST"))]
    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(serialize = "BID"))]
    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(serialize = "ASK"))]
    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(serialize = "ASK_BID"))]
    #[serde(rename(deserialize = "ASK_BID"))]
    AskBid,

    #[serde(rename(serialize = "MARK"))]
    #[serde(rename(deserialize = "MARK"))]
    Mark,

    #[serde(rename(serialize = "AVERAGE"))]
    #[serde(rename(deserialize = "AVERAGE"))]
    Average,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StopPriceLinkType {
    #[serde(rename(serialize = "VALUE"))]
    #[serde(rename(deserialize = "VALUE"))]
    Value,

    #[serde(rename(serialize = "PERCENT"))]
    #[serde(rename(deserialize = "PERCENT"))]
    Percent,

    #[serde(rename(serialize = "TICK"))]
    #[serde(rename(deserialize = "TICK"))]
    Tick,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StopType {
    #[serde(rename(serialize = "STANDARD"))]
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,

    #[serde(rename(serialize = "BID"))]
    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(serialize = "ASK"))]
    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(serialize = "LAST"))]
    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(serialize = "MARK"))]
    #[serde(rename(deserialize = "MARK"))]
    Mark,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PriceLinkBasis {
    #[serde(rename(serialize = "MANUAL"))]
    #[serde(rename(deserialize = "MANUAL"))]
    Manual,

    #[serde(rename(serialize = "BASE"))]
    #[serde(rename(deserialize = "BASE"))]
    Base,

    #[serde(rename(serialize = "TRIGGER"))]
    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,

    #[serde(rename(serialize = "LAST"))]
    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(serialize = "BID"))]
    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(serialize = "ASK"))]
    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(serialize = "ASK_BID"))]
    #[serde(rename(deserialize = "ASK_BID"))]
    AskBid,

    #[serde(rename(serialize = "MARK"))]
    #[serde(rename(deserialize = "MARK"))]
    Mark,

    #[serde(rename(serialize = "AVERAGE"))]
    #[serde(rename(deserialize = "AVERAGE"))]
    Average,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PriceLinkType {
    #[serde(rename(serialize = "VALUE"))]
    #[serde(rename(deserialize = "VALUE"))]
    Value,

    #[serde(rename(serialize = "PERCENT"))]
    #[serde(rename(deserialize = "PERCENT"))]
    Percent,

    #[serde(rename(serialize = "TICK"))]
    #[serde(rename(deserialize = "TICK"))]
    Tick,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TaxLotMethod {
    #[serde(rename(serialize = "FIFO"))]
    #[serde(rename(deserialize = "FIFO"))]
    Fifo,

    #[serde(rename(serialize = "LIFO"))]
    #[serde(rename(deserialize = "LIFO"))]
    Lifo,

    #[serde(rename(serialize = "HIGH_COST"))]
    #[serde(rename(deserialize = "HIGH_COST"))]
    HighCost,

    #[serde(rename(serialize = "LOW_COST"))]
    #[serde(rename(deserialize = "LOW_COST"))]
    LowCost,

    #[serde(rename(serialize = "AVERAGE_COST"))]
    #[serde(rename(deserialize = "AVERAGE_COST"))]
    AverageCost,

    #[serde(rename(serialize = "SPECIFIC_LOT"))]
    #[serde(rename(deserialize = "SPECIFIC_LOT"))]
    SpecificLot,

    #[serde(rename(serialize = "LOSS_HARVESTER"))]
    #[serde(rename(deserialize = "LOSS_HARVESTER"))]
    LossHarvester,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SpecialInstruction {
    #[serde(rename(serialize = "ALL_OR_NONE"))]
    #[serde(rename(deserialize = "ALL_OR_NONE"))]
    AllOrNone,

    #[serde(rename(serialize = "DO_NOT_REDUCE"))]
    #[serde(rename(deserialize = "DO_NOT_REDUCE"))]
    DoNotReduce,

    #[serde(rename(serialize = "ALL_OR_NONE_DO_NOT_REDUCE"))]
    #[serde(rename(deserialize = "ALL_OR_NONE_DO_NOT_REDUCE"))]
    AllOrNoneDoNotReduce,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderStrategyType {
    #[serde(rename(serialize = "SINGLE"))]
    #[serde(rename(deserialize = "SINGLE"))]
    Single,

    #[serde(rename(serialize = "CANCEL"))]
    #[serde(rename(deserialize = "CANCEL"))]
    Cancel,

    #[serde(rename(serialize = "RECALL"))]
    #[serde(rename(deserialize = "RECALL"))]
    Recall,

    #[serde(rename(serialize = "PAIR"))]
    #[serde(rename(deserialize = "PAIR"))]
    Pair,

    #[serde(rename(serialize = "FLATTEN"))]
    #[serde(rename(deserialize = "FLATTEN"))]
    Flatten,

    #[serde(rename(serialize = "TWO_DAY_SWAP"))]
    #[serde(rename(deserialize = "TWO_DAY_SWAP"))]
    TwoDaySwap,

    #[serde(rename(serialize = "BLAST_ALL"))]
    #[serde(rename(deserialize = "BLAST_ALL"))]
    BlastAll,

    #[serde(rename(serialize = "OCO"))]
    #[serde(rename(deserialize = "OCO"))]
    Oco,

    #[serde(rename(serialize = "TRIGGER"))]
    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderStatus {
    #[serde(rename(serialize = "AWAITING_PARENT_ORDER"))]
    #[serde(rename(deserialize = "AWAITING_PARENT_ORDER"))]
    AwaitingParentOrder,

    #[serde(rename(serialize = "AWAITING_CONDITION"))]
    #[serde(rename(deserialize = "AWAITING_CONDITION"))]
    AwaitingCondition,

    #[serde(rename(serialize = "AWAITING_STOP_CONDITION"))]
    #[serde(rename(deserialize = "AWAITING_STOP_CONDITION"))]
    AwaitingStopCondition,

    #[serde(rename(serialize = "AWAITING_MANUAL_REVIEW"))]
    #[serde(rename(deserialize = "AWAITING_MANUAL_REVIEW"))]
    AwaitingManualReview,

    #[serde(rename(serialize = "ACCEPTED"))]
    #[serde(rename(deserialize = "ACCEPTED"))]
    Accepted,

    #[serde(rename(serialize = "AWAITING_UR_OUT"))]
    #[serde(rename(deserialize = "AWAITING_UR_OUT"))]
    AwaitingUrOut,

    #[serde(rename(serialize = "PENDING_ACTIVATION"))]
    #[serde(rename(deserialize = "PENDING_ACTIVATION"))]
    PendingActivation,

    #[serde(rename(serialize = "QUEUED"))]
    #[serde(rename(deserialize = "QUEUED"))]
    Queued,

    #[serde(rename(serialize = "WORKING"))]
    #[serde(rename(deserialize = "WORKING"))]
    Working,

    #[serde(rename(serialize = "REJECTED"))]
    #[serde(rename(deserialize = "REJECTED"))]
    Rejected,

    #[serde(rename(serialize = "PENDING_CANCEL"))]
    #[serde(rename(deserialize = "PENDING_CANCEL"))]
    PendingCancel,

    #[serde(rename(serialize = "CANCELED"))]
    #[serde(rename(deserialize = "CANCELED"))]
    Canceled,

    #[serde(rename(serialize = "PENDING_REPLACE"))]
    #[serde(rename(deserialize = "PENDING_REPLACE"))]
    PendingReplace,

    #[serde(rename(serialize = "REPLACED"))]
    #[serde(rename(deserialize = "REPLACED"))]
    Replaced,

    #[serde(rename(serialize = "FILLED"))]
    #[serde(rename(deserialize = "FILLED"))]
    Filled,

    #[serde(rename(serialize = "EXPIRED"))]
    #[serde(rename(deserialize = "EXPIRED"))]
    Expired,

    #[serde(rename(serialize = "NEW"))]
    #[serde(rename(deserialize = "NEW"))]
    New,

    #[serde(rename(serialize = "AWAITING_RELEASE_TIME"))]
    #[serde(rename(deserialize = "AWAITING_RELEASE_TIME"))]
    AwaitingReleaseTime,

    #[serde(rename(serialize = "PENDING_ACKNOWLEDGEMENT"))]
    #[serde(rename(deserialize = "PENDING_ACKNOWLEDGEMENT"))]
    PendingAcknowledgement,

    #[serde(rename(serialize = "PENDING_RECALL"))]
    #[serde(rename(deserialize = "PENDING_RECALL"))]
    PendingRecall,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderInstruction {
    #[serde(rename(serialize = "BUY"))]
    #[serde(rename(deserialize = "BUY"))]
    Buy,

    #[serde(rename(serialize = "SELL"))]
    #[serde(rename(deserialize = "SELL"))]
    Sell,

    #[serde(rename(serialize = "BUY_TO_COVER"))]
    #[serde(rename(deserialize = "BUY_TO_COVER"))]
    BuyToCover,

    #[serde(rename(serialize = "SELL_SHORT"))]
    #[serde(rename(deserialize = "SELL_SHORT"))]
    SellShort,

    #[serde(rename(serialize = "BUY_TO_OPEN"))]
    #[serde(rename(deserialize = "BUY_TO_OPEN"))]
    BuyToOpen,

    #[serde(rename(serialize = "BUY_TO_CLOSE"))]
    #[serde(rename(deserialize = "BUY_TO_CLOSE"))]
    BuyToClose,

    #[serde(rename(serialize = "SELL_TO_OPEN"))]
    #[serde(rename(deserialize = "SELL_TO_OPEN"))]
    SellToOpen,

    #[serde(rename(serialize = "SELL_TO_CLOSE"))]
    #[serde(rename(deserialize = "SELL_TO_CLOSE"))]
    SellToClose,

    #[serde(rename(serialize = "EXCHANGE"))]
    #[serde(rename(deserialize = "EXCHANGE"))]
    Exchange,

    #[serde(rename(serialize = "SELL_SHORT_EXEMPT"))]
    #[serde(rename(deserialize = "SELL_SHORT_EXEMPT"))]
    SellShortExempt,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum QuantityType {
    #[serde(rename(serialize = "ALL_SHARES"))]
    #[serde(rename(deserialize = "ALL_SHARES"))]
    AllShares,

    #[serde(rename(serialize = "DOLLARS"))]
    #[serde(rename(deserialize = "DOLLARS"))]
    Dollars,

    #[serde(rename(serialize = "SHARES"))]
    #[serde(rename(deserialize = "SHARES"))]
    Shares,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DivCapGains {
    #[serde(rename(serialize = "REINVEST"))]
    #[serde(rename(deserialize = "REINVEST"))]
    Reinvest,

    #[serde(rename(serialize = "PAYOUT"))]
    #[serde(rename(deserialize = "PAYOUT"))]
    Payout,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum AccountsInstrument {
    AccountCashEquivalent {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
        r#type: TransactionCashEquivalentType,
    },

    AccountEquity {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
    },

    AccountCollectiveInvestment {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        r#type: CollectiveInvestmentType,
    },

    AccountFixedIncome {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
        #[serde(rename(serialize = "maturityDate"))]
        #[serde(rename(deserialize = "maturityDate"))]
        maturity_date: String,
        factor: f64,
        #[serde(rename(serialize = "variableRate"))]
        #[serde(rename(deserialize = "variableRate"))]
        variable_rate: f64,
    },

    AccountMutualFund {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
    },

    AccountOption {
        #[serde(rename(serialize = "assetType"))]
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        #[serde(rename(serialize = "instrumentId"))]
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(serialize = "netChange"))]
        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,
        #[serde(rename(serialize = "optionDeliverables"))]
        #[serde(rename(deserialize = "optionDeliverables"))]
        option_deliverables: Vec<AccountApiOptionDeliverable>,
        #[serde(rename(serialize = "putCall"))]
        #[serde(rename(deserialize = "putCall"))]
        pub_call: PutCallType,
        #[serde(rename(serialize = "optionMultiplier"))]
        #[serde(rename(deserialize = "optionMultiplier"))]
        option_multiplier: i32,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        r#type: TransactionOptionType,
        #[serde(rename(serialize = "underlyingSymbol"))]
        #[serde(rename(deserialize = "underlyingSymbol"))]
        underlying_sumbol: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ApiCurrencyType {
    #[serde(rename(serialize = "USD"))]
    #[serde(rename(deserialize = "USD"))]
    Usd,
    #[serde(rename(serialize = "CAD"))]
    #[serde(rename(deserialize = "CAD"))]
    Cad,
    #[serde(rename(serialize = "EUR"))]
    #[serde(rename(deserialize = "EUR"))]
    Eur,
    #[serde(rename(serialize = "JPY"))]
    #[serde(rename(deserialize = "JPY"))]
    Jpy,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountApiOptionDeliverable {
    pub symbol: String,
    pub deliverable_units: f64,
    pub api_currency_type: ApiCurrencyType,
    pub asset_type: AssetType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: AssetType,
    pub leg_id: i64,
    pub instrument: AccountsInstrument,
    pub instruction: OrderInstruction,
    pub position_effect: PositionEffect,
    pub quantity: f64,
    pub quantity_type: Option<QuantityType>,
    pub div_cap_gains: Option<DivCapGains>,
    pub to_symbol: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: i64,
    pub price: f64,
    pub quantity: f64,
    pub mismarked_quantity: f64,
    pub instrument_id: i64,
    pub time: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderActivityType {
    #[serde(rename(serialize = "EXECUTION"))]
    #[serde(rename(deserialize = "EXECUTION"))]
    Execution,
    #[serde(rename(serialize = "ORDER_ACTION"))]
    #[serde(rename(deserialize = "ORDER_ACTION"))]
    OrderAction,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderExecutionType {
    #[serde(rename(serialize = "FILL"))]
    #[serde(rename(deserialize = "FILL"))]
    Fill,

    #[serde(rename(serialize = "CANCELED"))]
    #[serde(rename(deserialize = "CANCELED"))]
    Canceled,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivity {
    pub activity_type: OrderActivityType,
    pub execution_type: OrderExecutionType,
    pub quantity: f64,
    pub order_remaining_quantity: f64,
    pub execution_legs: Vec<ExecutionLeg>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub session: OrderSession,        //
    pub order_type: OrderTypeRequest, //
    pub cancel_time: Option<String>,  // this is a date time, figure out how to auto parse this.
    pub duration: OrderDuration,      //
    pub complex_order_strategy_type: Option<ComplexOrderStrategyType>,
    pub quantity: Option<f64>,
    pub filled_quantity: Option<f64>,
    pub remaining_quantity: Option<f64>,
    pub destination_link_name: Option<String>,
    pub release_time: Option<String>,
    pub stop_price: Option<f64>,
    pub stop_price_link_basis: Option<StopPriceLinkBasis>,
    pub stop_price_link_type: Option<StopPriceLinkType>,
    pub stop_price_offset: Option<f64>,
    pub stop_type: Option<StopType>,
    pub price_link_basis: Option<PriceLinkBasis>,
    pub price_link_type: Option<PriceLinkType>,
    pub price: Option<f64>,
    pub tax_lot_method: Option<TaxLotMethod>,
    pub order_leg_collection: OrderLegCollection, //
    pub activation_price: Option<f64>,
    pub special_instruction: Option<SpecialInstruction>,
    pub order_strategy_type: OrderStrategyType, //
    pub order_id: Option<i64>,
    pub cancelable: Option<bool>,
    pub editable: Option<bool>,
    pub status: Option<OrderStatus>,
    pub entered_time: Option<String>,
    pub close_time: Option<String>,
    pub account_number: Option<i64>,
    pub order_activity_collection: Option<Vec<OrderActivity>>,
    pub status_description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderDuration {
    #[serde(rename(serialize = "DAY"))]
    #[serde(rename(deserialize = "DAY"))]
    Day,

    #[serde(rename(serialize = "GOOD_TILL_CANCEL"))]
    #[serde(rename(deserialize = "GOOD_TILL_CANCEL"))]
    GoodTillCancel,

    #[serde(rename(serialize = "FILL_OR_KILL"))]
    #[serde(rename(deserialize = "FILL_OR_KILL"))]
    FillOrKill,

    #[serde(rename(serialize = "IMMEDIATE_OR_CANCEL"))]
    #[serde(rename(deserialize = "IMMEDIATE_OR_CANCEL"))]
    ImmediateOrCancel,

    #[serde(rename(serialize = "END_OF_WEEK"))]
    #[serde(rename(deserialize = "END_OF_WEEK"))]
    EndOfWeek,

    #[serde(rename(serialize = "END_OF_MONTH"))]
    #[serde(rename(deserialize = "END_OF_MONTH"))]
    EndOfMonth,

    #[serde(rename(serialize = "NEXT_END_OF_MONTH"))]
    #[serde(rename(deserialize = "NEXT_END_OF_MONTH"))]
    NextEndOfMonth,

    #[serde(rename(serialize = "UNKNOWN"))]
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RequestedDestination {
    #[serde(rename(serialize = "INET"))]
    #[serde(rename(deserialize = "INET"))]
    Inet,

    #[serde(rename(serialize = "ECN_ARCA"))]
    #[serde(rename(deserialize = "ECN_ARCA"))]
    EcnArca,

    #[serde(rename(serialize = "CBOE"))]
    #[serde(rename(deserialize = "CBOE"))]
    Cboe,

    #[serde(rename(serialize = "AMEX"))]
    #[serde(rename(deserialize = "AMEX"))]
    Amex,

    #[serde(rename(serialize = "PHLX"))]
    #[serde(rename(deserialize = "PHLX"))]
    Phlx,

    #[serde(rename(serialize = "ISE"))]
    #[serde(rename(deserialize = "ISE"))]
    Ise,

    #[serde(rename(serialize = "BOX"))]
    #[serde(rename(deserialize = "BOX"))]
    Box,

    #[serde(rename(serialize = "NYSE"))]
    #[serde(rename(deserialize = "NYSE"))]
    Nyse,

    #[serde(rename(serialize = "NASDAQ"))]
    #[serde(rename(deserialize = "NASDAQ"))]
    Nasdaq,

    #[serde(rename(serialize = "BATS"))]
    #[serde(rename(deserialize = "BATS"))]
    Bats,

    #[serde(rename(serialize = "C2"))]
    #[serde(rename(deserialize = "C2"))]
    C2,

    #[serde(rename(serialize = "AUTO"))]
    #[serde(rename(deserialize = "AUTO"))]
    Auto,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub session: OrderSession,
    pub duration: OrderDuration,
    pub order_type: OrderTypeRequest,
    pub cancel_time: Option<String>, // this is a date time, figure out how to auto parse this.
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: RequestedDestination,
    pub destination_link_name: String,
    pub release_time: Option<String>,
    pub stop_price: Option<f64>,
    pub stop_price_link_basis: Option<StopPriceLinkBasis>,
    pub stop_price_link_type: Option<StopPriceLinkType>,
    pub stop_price_offset: Option<f64>,
    pub stop_type: Option<StopType>,
    pub price_link_basis: Option<PriceLinkBasis>,
    pub price_link_type: Option<PriceLinkType>,
    pub price: Option<f64>,
    pub tax_lot_method: Option<TaxLotMethod>,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub activation_price: Option<f64>,
    pub special_instruction: Option<SpecialInstruction>,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: OrderStatus,
    pub entered_time: String,
    pub close_time: String,
    pub account_number: i64,
    pub order_activity_collection: Vec<OrderActivity>,
    pub tag: Option<String>,
    pub status_description: Option<String>,
}

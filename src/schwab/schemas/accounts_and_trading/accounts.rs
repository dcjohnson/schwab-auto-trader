use serde::Deserialize;
use std::fmt;

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
    pub cash_account: Option<CashAccount>,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountApiOptionDeliverable {
    pub symbol: String,
    pub deliverable_units: f64,
    pub api_currency_type: String,
    pub asset_type: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {}

pub type Transactions = Vec<Transaction>;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
    Unknown,
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
    pub instrument: TransactionInstrument,
    pub amount: f64,
    pub cost: f64,
    pub price: Option<f64>,
    pub fee_type: Option<FeeType>,
    pub position_effect: Option<PositionEffect>,
}

#[derive(Deserialize, Debug)]
pub enum FeeType {
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
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum PositionEffect {
    #[serde(rename(deserialize = "OPENING"))]
    Opening,
    #[serde(rename(deserialize = "CLOSING"))]
    Closing,
    #[serde(rename(deserialize = "AUTOMATIC"))]
    Automatic,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum AssetType {
    #[serde(rename(deserialize = "EQUITY"))]
    Equity,
    #[serde(rename(deserialize = "OPTION"))]
    Option,
    #[serde(rename(deserialize = "INDEX"))]
    Index,
    #[serde(rename(deserialize = "MUTUAL_FUND"))]
    MutualFund,
    #[serde(rename(deserialize = "CASH_EQUIVALENT"))]
    CashEquivalent,
    #[serde(rename(deserialize = "FIXED_INCOME"))]
    FixedIncome,
    #[serde(rename(deserialize = "CURRENCY"))]
    Currency,
    #[serde(rename(deserialize = "COLLECTIVE_INVESTMENT"))]
    CollectiveInvestment,
}

#[derive(Deserialize, Debug)]
pub enum TransactionCashEquivalentType {
    #[serde(rename(deserialize = "SWEEP_VEHICLE"))]
    SweepVehicle,
    #[serde(rename(deserialize = "SAVINGS"))]
    Savings,
    #[serde(rename(deserialize = "MONEY_MARKET_FUND"))]
    MoneyMarketFund,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum CollectiveInvestmentType {
    #[serde(rename(deserialize = "UNIT_INVESTMENT_TRUST"))]
    UnitInvestmentTrust,
    #[serde(rename(deserialize = "EXCHANGE_TRADED_FUND"))]
    ExchangeTradedFund,
    #[serde(rename(deserialize = "CLOSED_END_FUND"))]
    ClosedEndFund,
    #[serde(rename(deserialize = "INDEX"))]
    Index,
    #[serde(rename(deserialize = "UNITS"))]
    Units,
}

#[derive(Deserialize, Debug)]
pub enum TransactionEquityType {
    #[serde(rename(deserialize = "COMMON_STOCK"))]
    CommonStock,
    #[serde(rename(deserialize = "PREFERRED_STOCK"))]
    PreferredStock,
    #[serde(rename(deserialize = "DEPOSITORY_RECEIPT"))]
    DepositoryReceipt,
    #[serde(rename(deserialize = "PREFERRED_DEPOSITORY_RECEIPT"))]
    PreferredDepositoryReceipt,
    #[serde(rename(deserialize = "RESTRICTED_STOCK"))]
    RestrictedStock,
    #[serde(rename(deserialize = "COMPONENT_UNIT"))]
    ComponentUnit,
    #[serde(rename(deserialize = "RIGHT"))]
    Right,
    #[serde(rename(deserialize = "WARRANT"))]
    Warrant,
    #[serde(rename(deserialize = "CONVERTIBLE_PREFERRED_STOCK"))]
    ConvertiblePreferredStock,
    #[serde(rename(deserialize = "CONVERTIBLE_STOCK"))]
    ConvertibleStock,
    #[serde(rename(deserialize = "LIMITED_PARTNERSHIP"))]
    LimitedPartnership,
    #[serde(rename(deserialize = "WHEN_ISSUED"))]
    WhenIssued,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum TransactionFixedIncomeType {
    #[serde(rename(deserialize = "BOND_UNIT"))]
    BondUnit,

    #[serde(rename(deserialize = "CERTIFICATE_OF_DEPOSIT"))]
    CertificateOfDeposit,

    #[serde(rename(deserialize = "CONVERTIBLE_BOND"))]
    ConvertibleBond,

    #[serde(rename(deserialize = "COLLATERALIZED_MORTGAGE_OBLIGATION"))]
    CollateralizedMortgageObligation,

    #[serde(rename(deserialize = "CORPORATE_BOND"))]
    CorporateBond,

    #[serde(rename(deserialize = "GOVERNMENT_MORTGAGE"))]
    GovernmentMortgage,

    #[serde(rename(deserialize = "GNMA_BONDS"))]
    GnmaBonds,

    #[serde(rename(deserialize = "MUNICIPAL_ASSESSMENT_DISTRICT"))]
    MunicipalAssessmentDistrict,

    #[serde(rename(deserialize = "MUNICIPAL_BOND"))]
    MunicipalBond,

    #[serde(rename(deserialize = "OTHER_GOVERNMENT"))]
    OtherGovernment,

    #[serde(rename(deserialize = "SHORT_TERM_PAPER"))]
    ShortTermPaper,

    #[serde(rename(deserialize = "US_TREASURY_BOND"))]
    UsTreasuryBond,

    #[serde(rename(deserialize = "US_TREASURY_BILL"))]
    UsTreasuryBill,

    #[serde(rename(deserialize = "US_TREASURY_NOTE"))]
    UsTreasuryNote,

    #[serde(rename(deserialize = "US_TREASURY_ZERO_COUPON"))]
    UsTreasuryZeroCoupon,

    #[serde(rename(deserialize = "AGENCY_BOND"))]
    AgencyBond,

    #[serde(rename(deserialize = "WHEN_AS_AND_IF_ISSUED_BOND"))]
    WhenAsAndIfissuedBond,

    #[serde(rename(deserialize = "ASSET_BACKED_SECURITY"))]
    AssetBackedSecurity,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum ForexType {
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,
    #[serde(rename(deserialize = "NBBO"))]
    Nbbo,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: f64,
}

#[derive(Deserialize, Debug)]
pub enum FutureType {
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum IndexType {
    #[serde(rename(deserialize = "BROAD_BASED"))]
    BroadBased,
    #[serde(rename(deserialize = "NARROW_BASED"))]
    NarrowBased,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum TransactionMutualFundType {
    #[serde(rename(deserialize = "NOT_APPLICABLE"))]
    NotApplicable,
    #[serde(rename(deserialize = "OPEN_END_NON_TAXABLE"))]
    OpenEndNonTaxable,
    #[serde(rename(deserialize = "OPEN_END_TAXABLE"))]
    OpenEndTaxable,
    #[serde(rename(deserialize = "NO_LOAD_NON_TAXABLE"))]
    NoLoadNonTaxable,
    #[serde(rename(deserialize = "NO_LOAD_TAXABLE"))]
    NoLoadTaxable,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum PutCallType {
    #[serde(rename(deserialize = "PUT"))]
    Put,
    #[serde(rename(deserialize = "CALL"))]
    Call,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum TransactionOptionType {
    #[serde(rename(deserialize = "VANILLA"))]
    Vanilla,
    #[serde(rename(deserialize = "BINARY"))]
    Binary,
    #[serde(rename(deserialize = "BARRIER"))]
    Barrier,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum ProductType {
    #[serde(rename(deserialize = "TBD"))]
    Tbd,
    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    // deliverable	Some empty type?
    pub deliverable: TransactionInstrument,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TransactionInstrument {
    TransactionEquity {
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        symbol: String,
        status: String,
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        #[serde(rename(deserialize = "type"))]
        te_type: TransactionEquityType,
    },

    CollectiveInvestment {
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        status: String,
        symbol: String,
        description: String,
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        #[serde(rename(deserialize = "type"))]
        ci_ype: CollectiveInvestmentType,
    },

    TransactionOption {
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        status: String,
        symbol: String,
        #[serde(rename(deserialize = "closingPrice"))]
        closing_price: f64,
        description: String,
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(deserialize = "expirationDate"))]
        expiration_date: String,
        #[serde(rename(deserialize = "optionDeliverables"))]
        option_deliverables: Vec<TransactionAPIOptionDeliverable>,
        #[serde(rename(deserialize = "optionPremiumMultiplier"))]
        option_premium_multiplier: i64,
        #[serde(rename(deserialize = "putCall"))]
        put_call: PutCallType,
        #[serde(rename(deserialize = "strikePrice"))]
        strike_price: f64,
        #[serde(rename(deserialize = "type"))]
        to_type: TransactionOptionType,
        #[serde(rename(deserialize = "underlyingSymbol"))]
        underlying_symbol: String,
        #[serde(rename(deserialize = "underlyingCusip"))]
        underlying_cusip: String,
    },

    Currency {
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,
        symbol: String,
        description: String,
        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,
        #[serde(rename(deserialize = "netChange"))]
        net_change: Option<f64>,
        status: String,
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

#[derive(Deserialize, Debug)]
pub enum OrderSession {
    #[serde(rename(deserialize = "NORMAL"))]
    Normal,

    #[serde(rename(deserialize = "AM"))]
    Am,

    #[serde(rename(deserialize = "PM"))]
    Pm,

    #[serde(rename(deserialize = "SEAMLESS"))]
    Seamless,
}

#[derive(Deserialize, Debug)]
pub enum OrderDuration {
    #[serde(rename(deserialize = "DAY"))]
    Day,

    #[serde(rename(deserialize = "GOOD_TILL_CANCEL"))]
    GoodTillCancel,

    #[serde(rename(deserialize = "FILL_OR_KILL"))]
    FillOrKill,

    #[serde(rename(deserialize = "IMMEDIATE_OR_CANCEL"))]
    ImmediateOrCancel,

    #[serde(rename(deserialize = "END_OF_WEEK"))]
    EndOfWeek,

    #[serde(rename(deserialize = "END_OF_MONTH"))]
    EndOfMonth,

    #[serde(rename(deserialize = "NEXT_END_OF_MONTH"))]
    NextEndOfMonth,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum OrderTypeRequest {
    #[serde(rename(deserialize = "MARKET"))]
    Market,

    #[serde(rename(deserialize = "LIMIT"))]
    Limit,

    #[serde(rename(deserialize = "STOP"))]
    Stop,

    #[serde(rename(deserialize = "STOP_LIMIT"))]
    StopLimit,

    #[serde(rename(deserialize = "TRAILING_STOP"))]
    TrailingStop,

    #[serde(rename(deserialize = "CABINET"))]
    Cabinet,

    #[serde(rename(deserialize = "NON_MARKETABLE"))]
    NonMarketable,

    #[serde(rename(deserialize = "MARKET_ON_CLOSE"))]
    MarketOnClose,

    #[serde(rename(deserialize = "EXERCISE"))]
    Exercise,

    #[serde(rename(deserialize = "TRAILING_STOP_LIMIT"))]
    TrailingStopLimit,

    #[serde(rename(deserialize = "NET_DEBIT"))]
    NetDebit,

    #[serde(rename(deserialize = "NET_CREDIT"))]
    NetCredit,

    #[serde(rename(deserialize = "NET_ZERO"))]
    NetZero,

    #[serde(rename(deserialize = "LIMIT_ON_CLOSE"))]
    LimitOnClose,
}

#[derive(Deserialize, Debug)]
pub enum ComplexOrderStrategyType {
    #[serde(rename(deserialize = "NONE"))]
    None,

    #[serde(rename(deserialize = "COVERED"))]
    Covered,

    #[serde(rename(deserialize = "VERTICAL"))]
    Vertical,

    #[serde(rename(deserialize = "BACK_RATIO"))]
    BackRatio,

    #[serde(rename(deserialize = "CALENDAR"))]
    Calendar,

    #[serde(rename(deserialize = "DIAGONAL"))]
    Diagonal,

    #[serde(rename(deserialize = "STRADDLE"))]
    Straddle,

    #[serde(rename(deserialize = "STRANGLE"))]
    Strangle,

    #[serde(rename(deserialize = "COLLAR_SYNTHETIC"))]
    CollarSynthetic,

    #[serde(rename(deserialize = "BUTTERFLY"))]
    Butterfly,

    #[serde(rename(deserialize = "CONDOR"))]
    Condor,

    #[serde(rename(deserialize = "IRON_CONDOR"))]
    IronCondor,

    #[serde(rename(deserialize = "VERTICAL_ROLL"))]
    VerticalRoll,

    #[serde(rename(deserialize = "COLLAR_WITH_STOCK"))]
    CollarWithStock,

    #[serde(rename(deserialize = "DOUBLE_DIAGONAL"))]
    DoubleDiagonal,

    #[serde(rename(deserialize = "UNBALANCED_BUTTERFLY"))]
    UnbalancedButterfly,

    #[serde(rename(deserialize = "UNBALANCED_CONDOR"))]
    UnbalancedCondor,

    #[serde(rename(deserialize = "UNBALANCED_IRON_CONDOR"))]
    UnbalancedIronCondor,

    #[serde(rename(deserialize = "UNBALANCED_VERTICAL_ROLL"))]
    UnbalancedVerticalRoll,

    #[serde(rename(deserialize = "MUTUAL_FUND_SWAP"))]
    MutualFundSwap,

    #[serde(rename(deserialize = "CUSTOM"))]
    Custom,
}

#[derive(Deserialize, Debug)]
pub enum StopPriceLinkBasis {
    #[serde(rename(deserialize = "MANUAL"))]
    Manual,

    #[serde(rename(deserialize = "BASE"))]
    Base,

    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,

    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(deserialize = "ASK_BID"))]
    AskBid,

    #[serde(rename(deserialize = "MARK"))]
    Mark,

    #[serde(rename(deserialize = "AVERAGE"))]
    Average,
}

#[derive(Deserialize, Debug)]
pub enum StopPriceLinkType {
    #[serde(rename(deserialize = "VALUE"))]
    Value,

    #[serde(rename(deserialize = "PERCENT"))]
    Percent,

    #[serde(rename(deserialize = "TICK"))]
    Tick,
}

#[derive(Deserialize, Debug)]
pub enum StopType {
    #[serde(rename(deserialize = "STANDARD"))]
    Standard,

    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(deserialize = "MARK"))]
    Mark,
}

#[derive(Deserialize, Debug)]
pub enum PriceLinkBasis {
    #[serde(rename(deserialize = "MANUAL"))]
    Manual,

    #[serde(rename(deserialize = "BASE"))]
    Base,

    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,

    #[serde(rename(deserialize = "LAST"))]
    Last,

    #[serde(rename(deserialize = "BID"))]
    Bid,

    #[serde(rename(deserialize = "ASK"))]
    Ask,

    #[serde(rename(deserialize = "ASK_BID"))]
    AskBid,

    #[serde(rename(deserialize = "MARK"))]
    Mark,

    #[serde(rename(deserialize = "AVERAGE"))]
    Average,
}

#[derive(Deserialize, Debug)]
pub enum PriceLinkType {
    #[serde(rename(deserialize = "VALUE"))]
    Value,

    #[serde(rename(deserialize = "PERCENT"))]
    Percent,

    #[serde(rename(deserialize = "TICK"))]
    Tick,
}

#[derive(Deserialize, Debug)]
pub enum TaxLotMethod {
    #[serde(rename(deserialize = "FIFO"))]
    Fifo,

    #[serde(rename(deserialize = "LIFO"))]
    Lifo,

    #[serde(rename(deserialize = "HIGH_COST"))]
    HighCost,

    #[serde(rename(deserialize = "LOW_COST"))]
    LowCost,

    #[serde(rename(deserialize = "AVERAGE_COST"))]
    AverageCost,

    #[serde(rename(deserialize = "SPECIFIC_LOT"))]
    SpecificLot,

    #[serde(rename(deserialize = "LOSS_HARVESTER"))]
    LossHarvester,
}

#[derive(Deserialize, Debug)]
pub enum SpecialInstruction {
    #[serde(rename(deserialize = "ALL_OR_NONE"))]
    AllOrNone,

    #[serde(rename(deserialize = "DO_NOT_REDUCE"))]
    DoNotReduce,

    #[serde(rename(deserialize = "ALL_OR_NONE_DO_NOT_REDUCE"))]
    AllOrNoneDoNotReduce,
}

#[derive(Deserialize, Debug)]
pub enum OrderStrategyType {
    #[serde(rename(deserialize = "SINGLE"))]
    Single,

    #[serde(rename(deserialize = "CANCEL"))]
    Cancel,

    #[serde(rename(deserialize = "RECALL"))]
    Recall,

    #[serde(rename(deserialize = "PAIR"))]
    Pair,

    #[serde(rename(deserialize = "FLATTEN"))]
    Flatten,

    #[serde(rename(deserialize = "TWO_DAY_SWAP"))]
    TwoDaySwap,

    #[serde(rename(deserialize = "BLAST_ALL"))]
    BlastAll,

    #[serde(rename(deserialize = "OCO"))]
    Oco,

    #[serde(rename(deserialize = "TRIGGER"))]
    Trigger,
}

#[derive(Deserialize, Debug)]
pub enum OrderStatus {
    #[serde(rename(deserialize = "AWAITING_PARENT_ORDER"))]
    AwaitingParentOrder,

    #[serde(rename(deserialize = "AWAITING_CONDITION"))]
    AwaitingCondition,

    #[serde(rename(deserialize = "AWAITING_STOP_CONDITION"))]
    AwaitingStopCondition,

    #[serde(rename(deserialize = "AWAITING_MANUAL_REVIEW"))]
    AwaitingManualReview,

    #[serde(rename(deserialize = "ACCEPTED"))]
    Accepted,

    #[serde(rename(deserialize = "AWAITING_UR_OUT"))]
    AwaitingUrOut,

    #[serde(rename(deserialize = "PENDING_ACTIVATION"))]
    PendingActivation,

    #[serde(rename(deserialize = "QUEUED"))]
    Queued,

    #[serde(rename(deserialize = "WORKING"))]
    Working,

    #[serde(rename(deserialize = "REJECTED"))]
    Rejected,

    #[serde(rename(deserialize = "PENDING_CANCEL"))]
    PendingCancel,

    #[serde(rename(deserialize = "CANCELED"))]
    Canceled,

    #[serde(rename(deserialize = "PENDING_REPLACE"))]
    PendingReplace,

    #[serde(rename(deserialize = "REPLACED"))]
    Replaced,

    #[serde(rename(deserialize = "FILLED"))]
    Filled,

    #[serde(rename(deserialize = "EXPIRED"))]
    Expired,

    #[serde(rename(deserialize = "NEW"))]
    New,

    #[serde(rename(deserialize = "AWAITING_RELEASE_TIME"))]
    AwaitingReleaseTime,

    #[serde(rename(deserialize = "PENDING_ACKNOWLEDGEMENT"))]
    PendingAcknowledgement,

    #[serde(rename(deserialize = "PENDING_RECALL"))]
    PendingRecall,

    #[serde(rename(deserialize = "UNKNOWN"))]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum OrderInstruction {
    #[serde(rename(deserialize = "BUY"))]
    Buy,

    #[serde(rename(deserialize = "SELL"))]
    Sell,

    #[serde(rename(deserialize = "BUY_TO_COVER"))]
    BuyToCover,

    #[serde(rename(deserialize = "SELL_SHORT"))]
    SellShort,

    #[serde(rename(deserialize = "BUY_TO_OPEN"))]
    BuyToOpen,

    #[serde(rename(deserialize = "BUY_TO_CLOSE"))]
    BuyToClose,

    #[serde(rename(deserialize = "SELL_TO_OPEN"))]
    SellToOpen,

    #[serde(rename(deserialize = "SELL_TO_CLOSE"))]
    SellToClose,

    #[serde(rename(deserialize = "EXCHANGE"))]
    Exchange,

    #[serde(rename(deserialize = "SELL_SHORT_EXEMPT"))]
    SellShortExempt,
}

#[derive(Deserialize, Debug)]
pub enum QuantityType {
    #[serde(rename(deserialize = "ALL_SHARES"))]
    AllShares,

    #[serde(rename(deserialize = "DOLLAR"))]
    Dollar,

    #[serde(rename(deserialize = "SHARES"))]
    Shares,
}

#[derive(Deserialize, Debug)]
pub enum DivCapGains {
    #[serde(rename(deserialize = "REINVEST"))]
    Reinvest,

    #[serde(rename(deserialize = "PAYOUT"))]
    Payout,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "assetType")] // why is this here? 
pub enum AccountsInstrument {
    #[serde(rename(deserialize = "ACCOUNT_CASH_EQUIVALENT"))]
    AccountCashEquivalent {
        #[serde(rename(deserialize = "assetType"))]
        asset_type: AssetType,

        cusip: string,

        symbol: String,

        description: String,

        #[serde(rename(deserialize = "instrumentId"))]
        instrument_id: i64,

        #[serde(rename(deserialize = "netChange"))]
        net_change: f64,

        r#type: TransactionCashEquivalentType,
    },

    #[serde(rename(deserialize = "ACCOUNT_EQUITY"))]
    AccountEquity {
        /*
        assetType*	string
        Enum:
        [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
        cusip	string
        symbol	string
        description	string
        instrumentId	integer($int64)
        netChange	number($double)

                 */
    },

    #[serde(rename(deserialize = "ACCOUNT_FIXED_INCOME"))]
    AccountFixedIncome {
        /*
        assetType*	string
        Enum:
        [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
        cusip	string
        symbol	string
        description	string
        instrumentId	integer($int64)
        netChange	number($double)
        maturityDate	string($date-time)
        factor	number($double)
        variableRate	number($double)
                */
    },

    #[serde(rename(deserialize = "ACCOUNT_MUTUAL_FUND"))]
    AccountMutualFund {
        /*
        assetType*	string
        Enum:
        [ EQUITY, OPTION, INDEX, MUTUAL_FUND, CASH_EQUIVALENT, FIXED_INCOME, CURRENCY, COLLECTIVE_INVESTMENT ]
        cusip	string
        symbol	string
        description	string
        instrumentId	integer($int64)
        netChange	number($double)
                */
    },

    #[serde(rename(deserialize = "ACCOUNT_OPTION"))]
    AccountOption {
        /*
        assetType*	string
        Enum:
        Array [ 8 ]
        cusip	string
        symbol	string
        description	string
        instrumentId	integer($int64)
        netChange	number($double)
        optionDeliverables	[
        xml: OrderedMap { "name": "optionDeliverables", "wrapped": true }
        AccountAPIOptionDeliverable{
        symbol	string($int64)
        deliverableUnits	number($double)
        apiCurrencyType	string
        Enum:
        [ USD, CAD, EUR, JPY ]
        assetType	assetTypestring
        Enum:
        [ EQUITY, MUTUAL_FUND, OPTION, FUTURE, FOREX, INDEX, CASH_EQUIVALENT, FIXED_INCOME, PRODUCT, CURRENCY, COLLECTIVE_INVESTMENT ]
        }]
        putCall	string
        Enum:
        [ PUT, CALL, UNKNOWN ]
        optionMultiplier	integer($int32)
        type	string
        Enum:
        [ VANILLA, BINARY, BARRIER, UNKNOWN ]
        underlyingSymbol	string
                */
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: AssetType,
    pub leg_id: i64,
    pub instrument: AccountsInstrument,
    pub instruction: OrderInstruction,
    pub position_effect: PositionEffect,
    pub quantity: f64,
    pub quantity_type: QuantityType,
    pub div_cap_gains: DivCapGains,
    pub to_symbol: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub session: OrderSession,
    pub order_type: OrderTypeRequest,
    pub cancel_time: String, // this is a date time, figure out how to auto parse this.
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub destination_link_name: String,
    pub release_time: String,
    pub stop_price: f64,
    pub stop_price_link_basis: StopPriceLinkBasis,
    pub stop_price_link_type: StopPriceLinkType,
    pub stop_price_offset: f64,
    pub stop_type: StopType,
    pub price_link_basis: PriceLinkBasis,
    pub price_link_type: PriceLinkType,
    pub price: f64,
    pub tax_lot_method: TaxLotMethod,
    pub order_leg_collection: OrderLegCollection,
    pub activation_price: f64,
    pub special_instruction: SpecialInstruction,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: OrderStatus,
    pub entered_time: String,
    pub close_time: String,
    pub account_number: i64,
    //pub    orderActivityCollection	[...]
    //   pub replacingOrderCollection	[...]
    //  pub  childOrderStrategies	[...]
    pub status_description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /*
    session	session[...]
    duration	duration[...]
    orderType	orderType[...]
    cancelTime	[...]
    complexOrderStrategyType	complexOrderStrategyType[...]
    quantity	[...]
    filledQuantity	[...]
    remainingQuantity	[...]
    requestedDestination	requestedDestination[...]
    destinationLinkName	[...]
    releaseTime	[...]
    stopPrice	[...]
    stopPriceLinkBasis	stopPriceLinkBasis[...]
    stopPriceLinkType	stopPriceLinkType[...]
    stopPriceOffset	[...]
    stopType	stopType[...]
    priceLinkBasis	priceLinkBasis[...]
    priceLinkType	priceLinkType[...]
    price	[...]
    taxLotMethod	taxLotMethod[...]
    orderLegCollection	[...]
    activationPrice	[...]
    specialInstruction	specialInstruction[...]
    orderStrategyType	orderStrategyType[...]
    orderId	[...]
    cancelable	[...]
    editable	[...]
    status	status[...]
    enteredTime	[...]
    closeTime	[...]
    tag	[...]
    accountNumber	[...]
    orderActivityCollection	[...]
    replacingOrderCollection	[...]
    childOrderStrategies	[...]
    statusDescription	[...]
        */
}

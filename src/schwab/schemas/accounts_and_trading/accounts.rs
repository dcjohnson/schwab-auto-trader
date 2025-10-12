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
    cd_domain_id: String,
    login: String,
    r#type: UserDetailsType,
    user_id: i64,
    system_user_name: String,
    first_name: String,
    last_name: String,
    broker_rep_code: String,
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
         Unknown ,
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
       asset_type: AssetType ,
       cusip: 	String,
       symbol :	String,
       description	: String,
       instrument_id :	i64 ,
       net_change :	f64 ,
}

#[derive(Deserialize, Debug)]
      pub enum FutureType{
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
        Unknown , 
       }




#[derive(Deserialize, Debug)]
         pub enum  PutCallType {
    #[serde(rename(deserialize = "PUT"))]
       Put,
    #[serde(rename(deserialize = "CALL"))]
       Call,
    #[serde(rename(deserialize = "UNKNOWN"))]
       Unknown ,
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
        Unknown ,
}

#[derive(Deserialize, Debug)]
    pub enum   ProductType {
    #[serde(rename(deserialize = "TBD"))]
        Tbd, 
    #[serde(rename(deserialize = "UNKNOWN"))]
        Unknown ,
       }






#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable{
       root_symbol	: String ,
       strike_percent	: i64   ,
       deliverable_number   : i64 , 
       deliverable_units   : f64, 
        // deliverable	Some empty type? 
       asset_type  : AssetType, 
       }









#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionInstrument {
    TransactionCashEquivalent {
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
        r#type: TransactionCashEquivalentType,
    },

    CollectiveInvestment {
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
        r#type: CollectiveInvestmentType,
    },
    Currency {
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
    },

    TransactionEquity {
        asset_type: AssetType,
        cusip: String,
        symbol: String,
        description: String,
        instrument_id: i64,
        net_change: f64,
        r#type: TransactionEquityType,
    }, 
       TransactionFixedIncome{
       asset_type: 	String,
       cusip	: String ,
       symbol	: String ,
       description	: String ,
       instrument_id: 	i64 ,
       net_change:	f64 ,
       r#type: TransactionFixedIncomeType, 
       maturity_cate:  String, 
       factor	: f64,
       multiplier :	f64, 
       variable_rate	: f64,
       },

    
       Forex{
       asset_type : AssetType ,
       cusip	: String ,
       symbol	: String ,
       description :	String ,
       instrument_id	: i64, 
       net_change    : f64, 
       r#type : ForexType, 
       base_currency : 	Currency,
       counter_currency	: Currency, 
       },

       Future{
       active_contract :bool, 
       r#type : FutureType , 
       expiration_date : String, 
       last_trading_date	: String, 
       first_notice_date	: String , 
       multiplier	: f64, 
       },

       Index{
       active_contract : bool, 
       r#type : IndexType , 
       },



       TransactionMutualFund{
       asset_type : AssetType, 
       cusip	: String,
       symbol	: String,
       description	: String,
       instrument_id	: i64 , 
       net_change	: f64 , 
       fund_family_name	: String,
       fund_family_symbol	: String,
       fund_group	: String ,
       r#type : TransactionMutualFundType, 
       exchange_cutoff_time : String, 
       purchase_cutoff_time	 : String , 
       redemption_cutoff_time	: String ,
       },
 
       TransactionOption{
       asset_type : AssetType, 
       cusip :	String,
       symbol :	String,
       description	: String,
       instrument_id :	i64,
       net_change :	f64 ,
       expiration_date :	String ,
       option_deliverables : Vec<TransactionAPIOptionDeliverable> , 
       option_premium_multiplier : 	i64 ,
       put_call : PutCallType, 
       strike_price : f64 ,
       r#type : TransactionOptionType , 
       underlying_symbol   :	String,
       underlying_cusip	 : String,
       // deliverable empty field? 
       },
 
       Product{
       asset_type : AssetType , 
       cusip :	String,
       symbol	: String,
       description :	String,
       instrument_id  :  f64,  
       net_change   :      f64, 
       r#type : ProductType, 
       },
       
}

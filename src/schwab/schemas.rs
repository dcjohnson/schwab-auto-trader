use serde::Deserialize;

pub type Accounts = Vec<Account>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    securities_account: SecuritiesAccount,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AccountInstrument {
AccountCashEquivalent {
asset_type: String, 
cusip: String, 
symbol: String, 
description: String, 
instrument_id: i64, 
net_change: f64,
type: String,
}, 
AccountEquity { 
}, 
AccountFixedIncome {
}, 
AccountMutualFund {
}, 
AccountOption {
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
              "instrument": {
                "cusip": "string",
                "symbol": "string",
                "description": "string",
                "instrumentId": 0,
                "netChange": 0,
                "type": "SWEEP_VEHICLE"
              },
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
pub struct SecuritiesAccount {
    account_number: String,
    round_trips: i32,
    is_day_trader: bool, 
    is_closing_only_restricted: bool, 
    pfcb_flag: bool, 
    positions: Vec<Position>,
          "initialBalances": {
            "accruedInterest": 0,
            "availableFundsNonMarginableTrade": 0,
            "bondValue": 0,
            "buyingPower": 0,
            "cashBalance": 0,
            "cashAvailableForTrading": 0,
            "cashReceipts": 0,
            "dayTradingBuyingPower": 0,
            "dayTradingBuyingPowerCall": 0,
            "dayTradingEquityCall": 0,
            "equity": 0,
            "equityPercentage": 0,
            "liquidationValue": 0,
            "longMarginValue": 0,
            "longOptionMarketValue": 0,
            "longStockValue": 0,
            "maintenanceCall": 0,
            "maintenanceRequirement": 0,
            "margin": 0,
            "marginEquity": 0,
            "moneyMarketFund": 0,
            "mutualFundValue": 0,
            "regTCall": 0,
            "shortMarginValue": 0,
            "shortOptionMarketValue": 0,
            "shortStockValue": 0,
            "totalCash": 0,
            "isInCall": 0,
            "unsettledCash": 0,
            "pendingDeposits": 0,
            "marginBalance": 0,
            "shortBalance": 0,
            "accountValue": 0
          },
          "currentBalances": {
            "availableFunds": 0,
            "availableFundsNonMarginableTrade": 0,
            "buyingPower": 0,
            "buyingPowerNonMarginableTrade": 0,
            "dayTradingBuyingPower": 0,
            "dayTradingBuyingPowerCall": 0,
            "equity": 0,
            "equityPercentage": 0,
            "longMarginValue": 0,
            "maintenanceCall": 0,
            "maintenanceRequirement": 0,
            "marginBalance": 0,
            "regTCall": 0,
            "shortBalance": 0,
              "shortQuantity": 0,
              "averagePrice": 0,
              "currentDayProfitLoss": 0,
              "currentDayProfitLossPercentage": 0,
              "longQuantity": 0,
              "settledLongQuantity": 0,
              "settledShortQuantity": 0,
              "agedQuantity": 0,
              "instrument": {
                "cusip": "string",
                "symbol": "string",
                "description": "string",
                "instrumentId": 0,
                "netChange": 0,
                "type": "SWEEP_VEHICLE"
              },
              "marketValue": 0,
              "maintenanceRequirement": 0,
              "averageLongPrice": 0,
              "averageShortPrice": 0,
              "taxLotAverageLongPrice": 0,
              "taxLotAverageShortPrice": 0,
              "longOpenProfitLoss": 0,
              "shortOpenProfitLoss": 0,
              "previousSessionLongQuantity": 0,
              "previousSessionShortQuantity": 0,
              "currentDayCost": 0
            "shortMarginValue": 0,
            "sma": 0,
            "isInCall": 0,
            "stockBuyingPower": 0,
            "optionBuyingPower": 0
          },
          "projectedBalances": {
            "availableFunds": 0,
            "availableFundsNonMarginableTrade": 0,
            "buyingPower": 0,
            "buyingPowerNonMarginableTrade": 0,
            "dayTradingBuyingPower": 0,
            "dayTradingBuyingPowerCall": 0,
            "equity": 0,
            "equityPercentage": 0,
            "longMarginValue": 0,
            "maintenanceCall": 0,
            "maintenanceRequirement": 0,
            "marginBalance": 0,
            "regTCall": 0,
            "shortBalance": 0,
            "shortMarginValue": 0,
            "sma": 0,
            "isInCall": 0,
            "stockBuyingPower": 0,
            "optionBuyingPower": 0
          }
        }
      }
    */
}
/*
 [
  {
    "securitiesAccount": {
      "accountNumber": String
      "roundTrips": 0,
      "isDayTrader": false,
      "isClosingOnlyRestricted": false,
      "pfcbFlag": false,
      "positions": [
        {
          "shortQuantity": 0,
          "averagePrice": 0,
          "currentDayProfitLoss": 0,
          "currentDayProfitLossPercentage": 0,
          "longQuantity": 0,
          "settledLongQuantity": 0,
          "settledShortQuantity": 0,
          "agedQuantity": 0,
          "instrument": {
            "cusip": "string",
            "symbol": "string",
            "description": "string",
            "instrumentId": 0,
            "netChange": 0,
            "type": "SWEEP_VEHICLE"
          },
          "marketValue": 0,
          "maintenanceRequirement": 0,
          "averageLongPrice": 0,
          "averageShortPrice": 0,
          "taxLotAverageLongPrice": 0,
          "taxLotAverageShortPrice": 0,
          "longOpenProfitLoss": 0,
          "shortOpenProfitLoss": 0,
          "previousSessionLongQuantity": 0,
          "previousSessionShortQuantity": 0,
          "currentDayCost": 0
        }
      ],
      "initialBalances": {
        "accruedInterest": 0,
        "availableFundsNonMarginableTrade": 0,
        "bondValue": 0,
        "buyingPower": 0,
        "cashBalance": 0,
        "cashAvailableForTrading": 0,
        "cashReceipts": 0,
        "dayTradingBuyingPower": 0,
        "dayTradingBuyingPowerCall": 0,
        "dayTradingEquityCall": 0,
        "equity": 0,
        "equityPercentage": 0,
        "liquidationValue": 0,
        "longMarginValue": 0,
        "longOptionMarketValue": 0,
        "longStockValue": 0,
        "maintenanceCall": 0,
        "maintenanceRequirement": 0,
        "margin": 0,
        "marginEquity": 0,
        "moneyMarketFund": 0,
        "mutualFundValue": 0,
        "regTCall": 0,
        "shortMarginValue": 0,
        "shortOptionMarketValue": 0,
        "shortStockValue": 0,
        "totalCash": 0,
        "isInCall": 0,
        "unsettledCash": 0,
        "pendingDeposits": 0,
        "marginBalance": 0,
        "shortBalance": 0,
        "accountValue": 0
      },
      "currentBalances": {
        "availableFunds": 0,
        "availableFundsNonMarginableTrade": 0,
        "buyingPower": 0,
        "buyingPowerNonMarginableTrade": 0,
        "dayTradingBuyingPower": 0,
        "dayTradingBuyingPowerCall": 0,
        "equity": 0,
        "equityPercentage": 0,
        "longMarginValue": 0,
        "maintenanceCall": 0,
        "maintenanceRequirement": 0,
        "marginBalance": 0,
        "regTCall": 0,
        "shortBalance": 0,
        "shortMarginValue": 0,
        "sma": 0,
        "isInCall": 0,
        "stockBuyingPower": 0,
        "optionBuyingPower": 0
      },
      "projectedBalances": {
        "availableFunds": 0,
        "availableFundsNonMarginableTrade": 0,
        "buyingPower": 0,
        "buyingPowerNonMarginableTrade": 0,
        "dayTradingBuyingPower": 0,
        "dayTradingBuyingPowerCall": 0,
        "equity": 0,
        "equityPercentage": 0,
        "longMarginValue": 0,
        "maintenanceCall": 0,
        "maintenanceRequirement": 0,
        "marginBalance": 0,
        "regTCall": 0,
        "shortBalance": 0,
        "shortMarginValue": 0,
        "sma": 0,
        "isInCall": 0,
        "stockBuyingPower": 0,
        "optionBuyingPower": 0
      }
    }
  }
]
*/

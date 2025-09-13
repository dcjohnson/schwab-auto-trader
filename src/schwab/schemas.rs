use serde::Deserialize;

pub type Accounts = Vec<Account>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    securities_account: SecuritiesAccount,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    account_number: String,
    round_trips: i32,
    /*
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

#![allow(unused)]
use async_trait::async_trait;
use std::{
    collections::{BTreeMap, HashMap},
    pin::Pin,
};

use futures::Future;
use reqwest::Method;
use serde_json::Value;

use crate::endpoints::v5spot_margin_trade;

use super::http_manager::{HttpManager, Manager};
#[async_trait]
pub trait SpotMarginTrade {
    fn new(http_manager: HttpManager) -> Self;
    async fn spot_margin_trade_toggle_margin_trade(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_set_leverage(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_margin_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_borrowable_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_interest_quota(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_loan_account_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_borrow(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_repay(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_borrow_order_detail(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_get_repayment_order_detail(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn spot_margin_trade_normal_toggle_margin_trade(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

pub struct SpotMarginTradeHTTP {
    http_manager: HttpManager,
}
#[async_trait]
impl SpotMarginTrade for SpotMarginTradeHTTP {
    ///
    ///
    //// Initialize the SpotMarginHTTP by passing the HttpManager
    ///
    ///
    fn new(http_manager: HttpManager) -> Self {
        SpotMarginTradeHTTP { http_manager }
    }
    // UTA only. Turn spot margin trade on / off.

    //     Required args:
    //         spotMarginMode (string): 1: on, 0: off

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-uta/switch-mode
    async fn spot_margin_trade_toggle_margin_trade(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5spot_margin_trade::SpotMarginTrade::ToggleMarginTrade.to_string();
        self.http_manager
            .submit_request(Method::POST, &endpoint, query, true)
            .await
    }

    // UTA only. Set the user's maximum leverage in spot cross margin

    //         Required args:
    //             leverage (string): Leverage. [2, 5].

    //         Returns:
    //             Request results as HashMap.

    //         Additional information:
    //             https://bybit-exchange.github.io/docs/v5/spot-margin-uta/set-leverage
    async fn spot_margin_trade_set_leverage(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5spot_margin_trade::SpotMarginTrade::SetLeverage.to_string();
        self.http_manager
            .submit_request(Method::POST, &endpoint, query, true)
            .await
    }

    // Normal (non-UTA) account only. Turn on / off spot margin trade

    //     Required args:
    //         switch (string): 1: on, 0: off

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/margin-data
    async fn spot_margin_trade_normal_get_margin_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5spot_margin_trade::SpotMarginTrade::NormalGetMarginCoinInfo.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/borrowable-data
    async fn spot_margin_trade_normal_get_borrowable_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint =
            v5spot_margin_trade::SpotMarginTrade::NormalGetBorrowableCoinInfo.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Required args:
    //         coin (string): Coin name

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/interest-quota
    async fn spot_margin_trade_normal_get_interest_quota(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5spot_margin_trade::SpotMarginTrade::NormalGetInterestQuota.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/account-info
    async fn spot_margin_trade_normal_get_loan_account_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5spot_margin_trade::SpotMarginTrade::NormalGetLoanAccountInfo.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Required args:
    //         coin (string): Coin name
    //         qty (string): Amount to borrow

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/borrow
    async fn spot_margin_trade_normal_borrow(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut endpoint = v5spot_margin_trade::SpotMarginTrade::NormalBorrow.to_string();
        endpoint = endpoint.replace("$instrument_id", &query["instrument_id"]);
        self.http_manager
            .submit_request(Method::POST, &endpoint, query, true)
            .await
    }

    // Normal (non-UTA) account only.

    //     Required args:
    //         coin (string): Coin name

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/repay
    async fn spot_margin_trade_normal_repay(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut endpoint = v5spot_margin_trade::SpotMarginTrade::NormalRepay.to_string();
        endpoint = endpoint.replace("$instrument_id", &query["instrument_id"]);
        self.http_manager
            .submit_request(Method::POST, &endpoint, query, true)
            .await
    }

    // Normal (non-UTA) account only.

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/borrow-order
    async fn spot_margin_trade_normal_get_borrow_order_detail(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut endpoint =
            v5spot_margin_trade::SpotMarginTrade::NormalGetBorrowOrderDetail.to_string();
        endpoint = endpoint.replace("$instrument_id", &query["instrument_id"]);
        endpoint = endpoint.replace("$borrow_id", &query["borrow_id"]);
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/repay-order
    async fn spot_margin_trade_normal_get_repayment_order_detail(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut endpoint =
            v5spot_margin_trade::SpotMarginTrade::NormalGetRepaymentOrderDetail.to_string();
        endpoint = endpoint.replace("$instrument_id", &query["instrument_id"]);
        endpoint = endpoint.replace("$repayment_id", &query["repayment_id"]);
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await
    }

    // Normal (non-UTA) account only.

    //     Required args:
    //         switch (integer): 1: on, 0: off

    //     Returns:
    //         Request results as HashMap.

    //     Additional information:
    //         https://bybit-exchange.github.io/docs/v5/spot-margin-normal/switch-mode
    async fn spot_margin_trade_normal_toggle_margin_trade(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut endpoint =
            v5spot_margin_trade::SpotMarginTrade::NormalToggleMarginTrade.to_string();
        endpoint = endpoint.replace("$instrument_id", &query["instrument_id"]);
        self.http_manager
            .submit_request(Method::POST, &endpoint, query, true)
            .await
    }
}

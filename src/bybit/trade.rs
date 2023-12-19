#![allow(unused)]
use async_trait::async_trait;
use std::{
    collections::{BTreeMap, HashMap},
    pin::Pin,
    sync::Arc,
};

use futures::Future;
use reqwest::Method;
use serde_json::Value;

use crate::endpoints::v5trade;

use super::{
    Result,
    http_manager::{HttpManager, Manager}
};
#[async_trait]
pub trait Trade {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn place_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
    async fn amend_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn cancel_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_open_orders(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn cancel_all_orders(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_order_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn amend_batch_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn cancel_batch_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
    async fn get_borrow_quota(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn set_dcp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
}
pub struct TradeHTTP {
    http_manager: Arc<HttpManager>,
}
#[async_trait]
impl Trade for TradeHTTP {
    ///
    ///
    //// Initialize the TradeHTTP by passing the HttpManager
    ///
    ///
    fn new(http_manager: Arc<HttpManager>) -> Self {
        TradeHTTP { http_manager }
    }
    ////
    /// This method supports to create the order for spot, spot margin, linear perpetual, inverse futures and options.
    /// Required args:
    ///     category (string): Product type Unified account: spot, linear, optionNormal account: linear, inverse. Please note that category is not involved with business logic
    ///     symbol (string): Symbol name
    ///     side (string): Buy, Sell
    ///     orderType (string): Market, Limit
    ///     qty (string): Order quantity
    /// Returns:
    ///     Request results as JSON data.
    /// Additional information:
    ///     https://bybit-exchange.github.io/docs/v5/order/create-order
    async fn place_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint = v5trade::Trade::PlaceOrder.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await
    }

    /// Unified account covers: Linear contract / Options
    ///     Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures

    ///     Required args:
    ///         category (string): Product type Unified account: spot, linear, optionNormal account: linear, inverse. Please note that category is not involved with business logic
    ///         symbol (string): Symbol name

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    async fn amend_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::AmendOrder.to_string(),
                true,
                query,
            )
            .await
    }

    /// Unified account covers: Spot / Linear contract / Options
    ///     Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures

    ///     Required args:
    ///         category (string): Product type Unified account: spot, linear, optionNormal account: linear, inverse. Please note that category is not involved with business logic
    ///         symbol (string): Symbol name
    ///         orderId (string): Order ID. Either orderId or orderLinkId is required
    ///         orderLinkId (string): User customised order ID. Either orderId or orderLinkId is required

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/cancel-order
    async fn cancel_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::CancelOrder.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query unfilled or partially filled orders in real-time. To query older order records, please use the order history interface.

    ///     Required args:
    ///         category (string): Product type Unified account: spot, linear, optionNormal account: linear, inverse. Please note that category is not involved with business logic

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/open-order
    async fn get_open_orders(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5trade::Trade::GetOpenOrders.to_string(),
                query,
                true,
            )
            .await
    }

    /// Cancel all open orders

    ///     Required args:
    ///         category (string): Product type
    ///             Unified account: spot, linear, option
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic. If cancel all by baseCoin, it will cancel all linear & inverse orders

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/cancel-all
    async fn cancel_all_orders(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::CancelAllOrders.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query order history. As order creation/cancellation is asynchronous, the data returned from this endpoint may delay.
    ///     If you want to get real-time order information, you could query this endpoint or rely on the websocket stream (recommended).

    ///     Required args:
    ///         category (string): Product type
    ///             Unified account: spot, linear, option
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/order-list
    async fn get_order_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5trade::Trade::GetOrderHistory.to_string(),
                query,
                true,
            )
            .await
    }
    /// Covers: Option (Unified Account)

    ///     Required args:
    ///         category (string): Product type. option
    ///         request (array): Object
    ///         > symbol (string): Symbol name
    ///         > side (string): Buy, Sell
    ///         > orderType (string): Market, Limit
    ///         > qty (string): Order quantity

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/batch-place
    async fn amend_batch_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::BatchAmendOrder.to_string(),
                true,
                query,
            )
            .await
    }
    /// This endpoint allows you to cancel more than one open order in a single request.

    ///     Required args:
    ///         category (string): Product type. option
    ///         request (array): Object
    ///         > symbol (string): Symbol name

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/batch-cancel
    async fn cancel_batch_order(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::CancelAllOrders.to_string(),
                true,
                query,
            )
            .await
    }
    /// Query the qty and amount of borrowable coins in spot account.

    ///     Required args:
    ///         category (string): Product type. spot
    ///         symbol (string): Symbol name
    ///         side (string): Transaction side. Buy,Sell

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/spot-borrow-quota
    async fn get_borrow_quota(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5trade::Trade::GetBorrowQuota.to_string(),
                query,
                true,
            )
            .await
    }
    /// Covers: Option (Unified Account)

    ///     Required args:
    ///         timeWindow (integer): Disconnection timing window time. [10, 300], unit: second

    ///     Returns:
    ///         Request results as JSON data.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/order/dcp
    async fn set_dcp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5trade::Trade::SetDcp.to_string(),
                true,
                query,
            )
            .await
    }
}

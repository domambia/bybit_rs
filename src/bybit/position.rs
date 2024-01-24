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

use crate::endpoints::v5position;

use super::{
    http_manager::{HttpManager, Manager},
    Result,
};

#[async_trait]
pub trait Position {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn get_position(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn set_leverage(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn switch_margin_mode(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn set_tp_sl_mode(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn switch_position_mode(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn set_risk_limit(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn set_trading_stop(&self, query: HashMap<String, String>) -> Result<Value>;
    async fn set_auto_add_margin(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn get_executions(&self, query: HashMap<String, String>) -> Result<Value>;

    async fn get_closed_pnl(&self, query: HashMap<String, String>) -> Result<Value>;
}

pub struct PositionHTTP {
    http_manager: Arc<HttpManager>,
}

#[async_trait]
impl Position for PositionHTTP {
    fn new(http_manager: Arc<HttpManager>) -> Self {
        PositionHTTP { http_manager }
    }

    /// Query real-time position data, such as position size, cumulative realizedPNL.

    ///     Required args:
    ///         category (string): Product type
    ///             Unified account: linear, option
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position
    async fn get_position(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::GetPositions.to_string();
        self.http_manager
            .submit_request(Method::GET, &path, query, true)
            .await
    }

    ///Set the leverage

    ///    Required args:
    ///        category (string): Product type
    ///            Unified account: linear
    ///            Normal account: linear, inverse.

    ///            Please note that category is not involved with business logic
    ///        symbol (string): Symbol name
    ///        buyLeverage (string): [0, max leverage of corresponding risk limit].
    ///            Note: Under one-way mode, buyLeverage must be the same as sellLeverage
    ///        sellLeverage (string): [0, max leverage of corresponding risk limit].
    ///            Note: Under one-way mode, buyLeverage must be the same as sellLeverage

    ///    Returns:
    ///        Request results as HashMap.

    ///    Additional information:
    ///        https://bybit-exchange.github.io/docs/v5/position/leverage
    async fn set_leverage(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SetLeverage.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }

    /// Select cross margin mode or isolated margin mode

    ///     Required args:
    ///         category (string): Product type. linear,inverse

    ///             Please note that category is not involved with business logicUnified account is not applicable
    ///         symbol (string): Symbol name
    ///         tradeMode (integer): 0: cross margin. 1: isolated margin
    ///         buyLeverage (string): The value must be equal to sellLeverage value
    ///         sellLeverage (string): The value must be equal to buyLeverage value

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/cross-isolate
    async fn switch_margin_mode(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SwitchMarginMode.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }

    /// Set TP/SL mode to Full or Partial

    ///     Required args:
    ///         category (string): Product type
    ///             Unified account: linear
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic
    ///         symbol (string): Symbol name
    ///         tpSlMode (string): TP/SL mode. Full,Partial

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/tpsl-mode
    async fn set_tp_sl_mode(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SetTpSlMode.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }
    /// It supports to switch the position mode for USDT perpetual and Inverse futures.
    ///     If you are in one-way Mode, you can only open one position on Buy or Sell side.
    ///     If you are in hedge mode, you can open both Buy and Sell side positions simultaneously.

    ///     Required args:
    ///         category (string): Product type. linear,inverse

    ///             Please note that category is not involved with business logicUnified account is not applicable

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/position-mode
    async fn switch_position_mode(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SwitchPositionMode.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }
    /// The risk limit will limit the maximum position value you can hold under different margin requirements.
    ///     If you want to hold a bigger position size, you need more margin. This interface can set the risk limit of a single position.
    ///     If the order exceeds the current risk limit when placing an order, it will be rejected. Click here to learn more about risk limit.

    ///     Required args:
    ///         category (string): Product type
    ///             Unified account: linear
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic
    ///         symbol (string): Symbol name
    ///         riskId (integer): Risk limit ID

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/set-risk-limit
    async fn set_risk_limit(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SetRiskLimit.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }
    /// Set the trading stop condition
    ///    Required args:
    ///       category (string): Product type
    ///          Unified account: linear
    ///         Normal account: linear, inverse.
    ///        Please note that category is not involved with business logic
    ///       symbol (string): Symbol name
    ///  Returns:
    ///     Request results as HashMap.
    /// Additional information:
    /// https://bybit-exchange.github.io/docs/v5/position/trading-stop
    ///
    async fn set_trading_stop(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SetTradingStop.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }

    /// Turn on/off auto-add-margin for isolated margin position

    ///     Required args:
    ///         category (string): Product type. linear
    ///         symbol (string): Symbol name
    ///         autoAddMargin (integer): Turn on/off. 0: off. 1: on

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/add-margin
    async fn set_auto_add_margin(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::SetAutoAddMargin.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &path, true, query)
            .await
    }

    /// Query users' execution records, sorted by execTime in descending order

    ///     Required args:
    ///         category (string):
    ///             Product type Unified account: spot, linear, option
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/execution
    ///
    async fn get_executions(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::GetExecutions.to_string();
        self.http_manager
            .submit_request(Method::GET, &path, query, true)
            .await
    }

    /// Query user's closed profit and loss records. The results are sorted by createdTime in descending order.

    ///     Required args:
    ///         category (string):
    ///             Product type Unified account: linear
    ///             Normal account: linear, inverse.

    ///             Please note that category is not involved with business logic

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/position/close-pnl
    async fn get_closed_pnl(&self, query: HashMap<String, String>) -> Result<Value> {
        let path = v5position::Position::GetClosedPnl.to_string();
        self.http_manager
            .submit_request(Method::GET, &path, query, true)
            .await
    }
}

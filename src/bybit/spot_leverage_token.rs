#![allow(unused)]
use async_trait::async_trait;
use std::sync::Arc;
use std::{
    collections::{BTreeMap, HashMap},
    pin::Pin,
};

use futures::Future;
use reqwest::Method;
use serde_json::Value;

use crate::endpoints::v5spot_leverage_token;

use super::{
    Result,
    http_manager::{HttpManager, Manager}
};



#[async_trait]
pub trait SpotLeverageTokenTrade {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn get_leveraged_token_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_leveraged_token_market(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn purchase_leveraged_token(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn redeem_leveraged_token(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_purchase_redemption_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
}

pub struct SpotLeverageTokenTradeHTTP {
    http_manager: Arc<HttpManager>,
}
#[async_trait]
impl SpotLeverageTokenTrade for SpotLeverageTokenTradeHTTP {
    ///
    ///
    ///// Initialize the SpotLeverageTokenHTTP by passing the Arc<HttpManager>
    ///
    ///
    fn new(http_manager: Arc<HttpManager>) -> Self {
        SpotLeverageTokenTradeHTTP { http_manager }
    }

    /// Query leverage token information

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/lt/leverage-token-info
    async fn get_leveraged_token_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint = v5spot_leverage_token::SpotLeverageToken::GetLeveragedTokenInfo.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await
    }

    /// Get leverage token market information

    ///     Required args:
    ///         ltCoin (string): Abbreviation of the LT, such as BTC3L

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/lt/leverage-token-reference
    async fn get_leveraged_token_market(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint =
            v5spot_leverage_token::SpotLeverageToken::GetLeveragedTokenMarket.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await
    }

    /// Purchase levearge token

    ///     Required args:
    ///         ltCoin (string): Abbreviation of the LT, such as BTC3L
    ///         ltAmount (string): Purchase amount

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/lt/purchase
    async fn purchase_leveraged_token(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint = v5spot_leverage_token::SpotLeverageToken::Purchase.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await
    }

    /// Redeem leverage token

    ///     Required args:
    ///         ltCoin (string): Abbreviation of the LT, such as BTC3L
    ///         quantity (string): Redeem quantity of LT

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/lt/redeem
    async fn redeem_leveraged_token(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint = v5spot_leverage_token::SpotLeverageToken::Redeem.to_string();
        self.http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await
    }

    /// Get purchase or redeem history

    ///     Required args:

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/lt/order-record
    async fn get_purchase_redemption_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint =
            v5spot_leverage_token::SpotLeverageToken::GetPurchaseRedemptionRecords.to_string();
        self.http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await
    }
}

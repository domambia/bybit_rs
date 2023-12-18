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

use crate::endpoints::v5broker;

use super::{
    Result,
    http_manager::{HttpManager, Manager}
};

#[async_trait]
pub trait Broker {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn get_broker_earnings(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
}

pub struct BrokerHTTP {
    http_manager: Arc<HttpManager>,
}

#[async_trait]
impl Broker for BrokerHTTP {
    fn new(http_manager: Arc<HttpManager>) -> Self {
        BrokerHTTP { http_manager }
    }

    /// Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/broker/earning
    async fn get_broker_earnings(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        let endpoint = v5broker::Broker::GetBrokerEarnings.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }
}

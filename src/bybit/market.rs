#![allow(unused)]
use async_trait::async_trait;
use std::{
    collections::{BTreeMap, HashMap},
    pin::Pin,
};

use futures::Future;
use reqwest::Method;
use serde_json::Value;

use crate::endpoints::v5market;

use super::http_manager::{HttpManager, Manager};

pub trait Makert {
    fn new(http_manager: HttpManager) -> Self;
    fn get_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_mark_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_index_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_premium_index_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_instruments_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_orderbook(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_tickers(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn get_funding_rate_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_public_trade_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_open_interest(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_historical_volatility(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_insurance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_risk_limit(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn get_option_delivery_price(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

pub struct MarketHTTP {
    http_manager: HttpManager,
}

#[async_trait]
impl Market for HttpManager {
    ///
    ///
    //// Initialize the MarketHTTP by passing the HttpManager
    ///
    ///
    fn new(http_manager: HttpManager) -> Self {
        TradeHTTP { http_manager }
    }
    /// Query the kline data. Charts are returned in groups based on the requested interval.

    ///     Required args:
    ///         category (string): Product type: spot,linear,inverse
    ///         symbol (string): Symbol name
    ///         interval (string): Kline interval.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/kline
    async fn get_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::KLINE.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query the mark price kline data. Charts are returned in groups based on the requested interval.

    ///     Required args:
    ///         category (string): Product type. linear,inverse
    ///         symbol (string): Symbol name
    ///         interval (string): Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/mark-kline
    async fn get_mark_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::MARK_PRICE_KLINE.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query the index price kline data. Charts are returned in groups based on the requested interval.

    ///     Required args:
    ///         category (string): Product type. linear,inverse
    ///         symbol (string): Symbol name
    ///         interval (string): Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/index-kline
    async fn get_index_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::INDEX_PRICE_KLINE.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Retrieve the premium index price kline data. Charts are returned in groups based on the requested interval.

    ///     Required args:
    ///         category (string): Product type. linear
    ///         symbol (string): Symbol name
    ///         interval (string): Kline interval

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/preimum-index-kline
    async fn get_premium_index_price_kline(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::PREMIUM_INDEX_PRICE_KLINE.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query a list of instruments of online trading pair.

    ///     Required args:
    ///         category (string): Product type. spot,linear,inverse,option

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/instrument
    async fn get_instruments_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::INSTRUMENTS_INFO.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query orderbook data

    ///     Required args:
    ///         category (string): Product type. spot, linear, inverse, option
    ///         symbol (string): Symbol name

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/orderbook
    async fn get_orderbook(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::ORDERBOOK_L2_25.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.

    ///     Required args:
    ///         category (string): Product type. spot,linear,inverse,option

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/tickers
    async fn get_tickers(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::TICKERS.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query historical funding rate. Each symbol has a different funding interval.
    ///     For example, if the interval is 8 hours and the current time is UTC 12, then it returns the last funding rate, which settled at UTC 8.
    ///     To query the funding rate interval, please refer to instruments-info.

    ///     Required args:
    ///         category (string): Product type. linear,inverse
    ///         symbol (string): Symbol name

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/history-fund-rate
    async fn get_funding_rate_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::FUNDING_RATE_HISTORY.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Query recent public trading data in Bybit.

    ///     Required args:
    ///         category (string): Product type. spot,linear,inverse,option
    ///         symbol (string): Symbol name

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/recent-trade
    async fn get_public_trade_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let url = v5market::PUBLIC_TRADE_HISTORY.to_string();
        self.http_manager
            .request(Method::GET, url, query, true)
            .await?
    }
    /// Get open interest of each symbol.

    ///     Required args:
    ///         category (string): Product type. linear,inverse
    ///         symbol (string): Symbol name
    ///         intervalTime (string): Interval. 5min,15min,30min,1h,4h,1d

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/open-interest
    async fn get_open_interest(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(
                Method::GET,
                v5market::OPEN_INTEREST.to_string(),
                query,
                true,
            )
            .await?
    }

    /// Query option historical volatility

    ///     Required args:
    ///         category (string): Product type. option

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/iv
    async fn get_historical_volatility(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(
                Method::GET,
                v5market::HISTORICAL_VOLATILITY.to_string(),
                query,
                true,
            )
            .await?
    }

    /// Query Bybit insurance pool data (BTC/USDT/USDC etc).
    ///     The data is updated every 24 hours.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/insurance
    async fn get_insurance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(
                Method::GET,
                v5market::INSURANCE_FUND.to_string(),
                query,
                true,
            )
            .await?
    }
    /// Query risk limit of futures

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/risk-limit
    async fn get_risk_limit(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(Method::GET, v5market::RISK_LIMIT.to_string(), query, true)
            .await?
    }
    /// Query Bybit insurance pool data (BTC/USDT/USDC etc).
    ///     The data is updated every 24 hours.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/insurance
    async fn get_option_delivery_price(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(
                Method::GET,
                v5market::OPTION_DELIVERY_PRICE.to_string(),
                query,
                true,
            )
            .await?
    }

    /// Query risk limit of futures

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/market/risk-limit
    async fn get_option_risk(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.http_manager
            .request(Method::GET, v5market::OPTION_RISK.to_string(), query, true)
            .await?
    }
}

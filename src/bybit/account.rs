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

use crate::endpoints::v5account;

use super::http_manager::{HttpManager, Manager};
#[async_trait]
pub trait Account {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn get_wallet_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn upgrade_to_unified_trading_account(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_borrow_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_collateral_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_coin_greeks(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_fee_rates(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_account_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn get_transaction_log(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn set_margin_mode(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn set_mmp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn reset_mmp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_mmp_state(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

pub struct AccountHTTP {
    http_manager: Arc<HttpManager>,
}

#[async_trait]
impl Account for AccountHTTP {
    ///
    ///
    /// Initialize the AccountHTTP by passing the HttpManager
    ///
    ///
    fn new(http_manager: Arc<HttpManager>) -> Self {
        AccountHTTP { http_manager }
    }

    /// Obtain wallet balance, query asset information of each currency, and account risk rate information under unified margin mode.
    ///     By default, currency information with assets or liabilities of 0 is not returned.

    ///     Required args:
    ///         accountType (string): Account type
    ///             Unified account: UNIFIED
    ///             Normal account: CONTRACT

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/wallet-balance
    async fn get_wallet_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetWalletBalance.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Upgrade Unified Account

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/upgrade-unified-account
    async fn upgrade_to_unified_trading_account(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::UpgradeToUnifiedAccount.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Get interest records, sorted in reverse order of creation time.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/borrow-history
    async fn get_borrow_history(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetBorrowHistory.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Get the collateral information of the current unified margin account, including loan interest rate, loanable amount, collateral conversion rate, whether it can be mortgaged as margin, etc.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/collateral-info
    async fn get_collateral_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetCollateralInfo.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Get current account Greeks information

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/coin-greeks
    async fn get_coin_greeks(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetCoinGreeks.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await?;
        Ok(result)
    }

    /// Get the trading fee rate of derivatives.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/fee-rate
    async fn get_fee_rates(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetFeeRate.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, false)
            .await?;
        Ok(result)
    }

    /// Query the margin mode configuration of the account.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/account-info
    async fn get_account_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetAccountInfo.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Query transaction logs in Unified account.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/transaction-log
    async fn get_transaction_log(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetTransactionLog.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Default is regular margin mode. This mode is valid for USDT Perp, USDC Perp and USDC Option.

    ///     Required args:
    ///         setMarginMode (string): REGULAR_MARGIN, PORTFOLIO_MARGIN

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/set-margin-mode
    async fn set_margin_mode(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::SetMarginMode.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Market Maker Protection (MMP) is an automated mechanism designed to protect market makers (MM) against liquidity risks
    ///     and over-exposure in the market. It prevents simultaneous trade executions on quotes provided by the MM within a short time span.
    ///     The MM can automatically pull their quotes if the number of contracts traded for an underlying asset exceeds the configured
    ///     threshold within a certain time frame. Once MMP is triggered, any pre-existing MMP orders will be automatically canceled,
    ///     and new orders tagged as MMP will be rejected for a specific duration — known as the frozen period — so that MM can
    ///     reassess the market and modify the quotes.

    ///     Required args:
    ///         baseCoin (strin): Base coin
    ///         window (string): Time window (ms)
    ///         frozenPeriod (string): Frozen period (ms). "0" means the trade will remain frozen until manually reset
    ///         qtyLimit (string): Trade qty limit (positive and up to 2 decimal places)
    ///         deltaLimit (string): Delta limit (positive and up to 2 decimal places)

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/set-mmp
    async fn set_mmp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::SetMMP.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Once the mmp triggered, you can unfreeze the account by this endpoint

    ///     Required args:
    ///         baseCoin (string): Base coin

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/reset-mmp
    async fn reset_mmp(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::ResetMMP.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Get MMP state

    ///     Required args:
    ///         baseCoin (string): Base coin

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/account/get-mmp-state
    async fn get_mmp_state(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5account::Account::GetMMPState.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }
}

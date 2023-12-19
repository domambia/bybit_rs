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

use crate::endpoints::v5asset;

use super::{
    Result,
    http_manager::{HttpManager, Manager}
};


#[async_trait]
pub trait Asset {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn get_coin_exchange_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_option_delivery_record(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_usdc_contract_settlement(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_spot_asset_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_coins_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_coin_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_transferable_coin(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
    async fn create_internal_transfer(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_internal_transfer_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn enable_universal_transfer_for_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn create_universal_transfer(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_universal_transfer_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_allowed_deposit_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn set_deposit_account(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_sub_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_internal_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_master_deposit_address(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_sub_deposit_address(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_withdrawal_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn get_withdrawable_amount(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn withdraw(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;

    async fn cancel_withdrawal(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value>;
}

pub struct AssetHTTP {
    http_manager: Arc<HttpManager>,
}

#[async_trait]
impl Asset for AssetHTTP {
    ///
    ///
    /// Initialize the AssetHTTP by passing the HttpManager
    ///
    ///
    fn new(http_manager: Arc<HttpManager>) -> Self {
        AssetHTTP { http_manager }
    }

    /// Query the coin exchange records.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/exchange
    async fn get_coin_exchange_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::GetCoinExchangeRecords.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query option delivery records, sorted by deliveryTime in descending order

    ///     Required args:
    ///         category (string): Product type. option

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/option-delivery
    async fn get_option_delivery_record(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::GetOptionDeliveryRecord.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query session settlement records of USDC perpetual and futures

    ///     Required args:
    ///         category (string): Product type. linear

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/settlement
    async fn get_usdc_contract_settlement(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::GetUsdcContractSettlement.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query asset information

    ///     Required args:
    ///         accountType (string): Account type. SPOT

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/asset-info
    async fn get_spot_asset_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetSpotAssetInfo.to_string(),
                query,
                true,
            )
            .await
    }

    /// You could get all coin balance of all account types under the master account, and sub account.

    ///     Required args:
    ///         memberId (string): User Id. It is required when you use master api key to check sub account coin balance
    ///         accountType (string): Account type

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/all-balance
    async fn get_coins_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetAllCoinsBalance.to_string(),
                query,
                true,
            )
            .await
    }
    /// Query the balance of a specific coin in a specific account type. Supports querying sub UID's balance.

    ///     Required args:
    ///         memberId (string): UID. Required when querying sub UID balance
    ///         accountType (string): Account type

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/account-coin-balance
    async fn get_coin_balance(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetSingleCoinBalance.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query the transferable coin list between each account type

    ///     Required args:
    ///         fromAccountType (string): From account type
    ///         toAccountType (string): To account type

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/transferable-coin
    async fn get_transferable_coin(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetTransferableCoin.to_string(),
                query,
                true,
            )
            .await
    }

    /// Create the internal transfer between different account types under the same UID.

    ///     Required args:
    ///         transferId (string): UUID. Please manually generate a UUID
    ///         coin (string): Coin
    ///         amount (string): Amount
    ///         fromAccountType (string): From account type
    ///         toAccountType (string): To account type

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/create-inter-transfer
    async fn create_internal_transfer(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::CreateInternalTransfer.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query the internal transfer records between different account types under the same UID.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/inter-transfer-list
    async fn get_internal_transfer_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetInternalTransferRecords.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query the sub UIDs under a main UID

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/sub-uid-list
    async fn get_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetSubUid.to_string(),
                query,
                true,
            )
            .await
    }

    /// Transfer between sub-sub or main-sub

    ///     Required args:
    ///         subMemberIds (array): This list has a single item. Separate multiple UIDs by comma, e.g., "uid1,uid2,uid3"

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/enable-unitransfer-subuid
    async fn enable_universal_transfer_for_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::EnableUtForSubUid.to_string(),
                true,
                query,
            )
            .await
    }

    /// Transfer between sub-sub or main-sub. Please make sure you have enabled universal transfer on your sub UID in advance.

    ///     Required args:
    ///         transferId (string): UUID. Please manually generate a UUID
    ///         coin (string): Coin
    ///         amount (string): Amount
    ///         fromMemberId (integer): From UID
    ///         toMemberId (integer): To UID
    ///         fromAccountType (string): From account type
    ///         toAccountType (string): To account type

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/unitransfer
    async fn create_universal_transfer(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::CreateUniversalTransfer.to_string(),
                true,
                query,
            )
            .await
    }

    /// Query universal transfer records

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/unitransfer-list
    async fn get_universal_transfer_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetUniversalTransferRecords.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query allowed deposit coin information. To find out paired chain of coin, please refer coin info api.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/deposit-coin-spec
    async fn get_allowed_deposit_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetAllowedDepositCoinInfo.to_string(),
                query,
                true,
            )
            .await
    }

    /// Set auto transfer account after deposit. The same function as the setting for Deposit on web GUI

    ///     Required args:
    ///         accountType (string): Account type: UNIFIED,SPOT,OPTION,CONTRACT,FUND

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/set-deposit-acct

    async fn set_deposit_account(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::SetDepositAccount.to_string(),
                true,
                query,
            )
            .await
    }
    /// Query deposit records.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/deposit-record

    async fn get_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetDepositRecords.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query subaccount's deposit records by MAIN UID's API key.

    ///     Required args:
    ///         subMemberId (string): Sub UID

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/sub-deposit-record
    async fn get_sub_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetSubAccountDepositRecords.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query deposit records within the Bybit platform. These transactions are not on the blockchain.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/internal-deposit-record
    async fn get_internal_deposit_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetInternalDepositRecords.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query the deposit address information of MASTER account.

    ///     Required args:
    ///         coin (string): Coin

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/master-deposit-addr
    async fn get_master_deposit_address(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetMasterDepositAddress.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query the deposit address information of SUB account.

    ///     Required args:
    ///         coin (string): Coin
    ///         chainType (string): Chain, e.g.,ETH

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/sub-deposit-addr
    async fn get_sub_deposit_address(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetSubDepositAddress.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query coin information, including chain information, withdraw and deposit status.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/coin-info
    async fn get_coin_info(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetCoinInfo.to_string(),
                query,
                true,
            )
            .await
    }

    /// Query withdrawal records.

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/withdraw-record
    async fn get_withdrawal_records(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetWithdrawalRecords.to_string(),
                query,
                true,
            )
            .await
    }
    /// Get withdrawable amount

    ///     Required args:
    ///         coin (string): Coin name

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/delay-amount
    async fn get_withdrawable_amount(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_request(
                Method::GET,
                &v5asset::Asset::GetWithdrawableAmount.to_string(),
                query,
                true,
            )
            .await
    }

    /// Withdraw assets from your Bybit account. You can make an off-chain transfer if the target wallet address is from Bybit. This means that no blockchain fee will be charged.

    ///     Required args:
    ///         coin (string): Coin
    ///         chain (string): Chain
    ///         address (string): Wallet address
    ///         tag (string): Tag. Required if tag exists in the wallet address list
    ///         amount (string): Withdraw amount
    ///         timestamp (integer): Current timestamp (ms). Used for preventing from withdraw replay

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/withdraw
    async fn withdraw(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::Withdraw.to_string(),
                true,
                query,
            )
            .await
    }

    /// Cancel the withdrawal

    ///     Required args:
    ///         id (string): Withdrawal ID

    ///     Returns:
    ///         Request results as dictionary.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/asset/cancel-withdraw
    async fn cancel_withdrawal(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value> {
        self.http_manager
            .submit_post_request(
                Method::POST,
                &v5asset::Asset::CancelWithdrawal.to_string(),
                true,
                query,
            )
            .await
    }
}

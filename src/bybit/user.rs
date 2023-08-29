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

use crate::endpoints::v5user;

use super::http_manager::{HttpManager, Manager};
#[async_trait]
pub trait User {
    fn new(http_manager: Arc<HttpManager>) -> Self;
    async fn create_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn create_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_sub_uid_list(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn freeze_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_api_key_information(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn modify_master_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn modify_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn delete_master_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn delete_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

pub struct UserHTTP {
    http_manager: Arc<HttpManager>,
}

#[async_trait]
impl User for UserHTTP {
    ///
    ///
    //// Initialize the UserHTTP by passing the Arc<HttpManager>
    ///
    ///
    fn new(http_manager: Arc<HttpManager>) -> Self {
        UserHTTP { http_manager }
    }

    /// Create a new sub user id. Use master user's api key only.

    ///     Required args:
    ///         username (string): Give a username of the new sub user id. 6-16 characters, must include both numbers and letters.cannot be the same as the exist or deleted one.
    ///         memberType (integer): 1: normal sub account, 6: custodial sub account

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/create-subuid
    async fn create_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::CreateSubUid.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// To create new API key for those newly created sub UID. Use master user's api key only.

    ///     Required args:
    ///         subuid (integer): Sub user Id
    ///         readOnly (integer): 0: Read and Write. 1: Read only
    ///         permissions (Object): Tick the types of permission. one of below types must be passed, otherwise the error is thrown

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/create-subuid-apikey
    async fn create_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::CreateSubApiKey.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }
    /// Get all sub uid of master account. Use master user's api key only.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/subuid-list
    async fn get_sub_uid_list(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::GetSubUidList.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Froze sub uid. Use master user's api key only.

    ///     Required args:
    ///         subuid (integer): Sub user Id
    ///         frozen (integer): 0: unfreeze, 1: freeze

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/froze-subuid
    async fn freeze_sub_uid(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::FreezeSubUid.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }

    /// Get the information of the api key. Use the api key pending to be checked to call the endpoint. Both master and sub user's api key are applicable.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/apikey-info
    async fn get_api_key_information(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::GetApiKeyInformation.to_string();
        let result = self
            .http_manager
            .submit_request(Method::GET, &endpoint, query, true)
            .await?;
        Ok(result)
    }
    /// Modify the settings of master api key. Use the api key pending to be modified to call the endpoint. Use master user's api key only.

    /// Required args:
    ///     permissions (Object): Tick the types of permission. one of below types must be passed, otherwise the error is thrown

    /// Returns:
    ///     Request results as HashMap.

    /// Additional information:
    ///     https://bybit-exchange.github.io/docs/v5/user/modify-master-apikey

    async fn modify_master_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::ModifyMasterApiKey.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Modify the settings of sub api key. Use the api key pending to be modified to call the endpoint. Use sub user's api key only.

    ///     Required args:
    ///         permissions (Object): Tick the types of permission. one of below types must be passed, otherwise the error is thrown

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/modify-sub-apikey

    async fn modify_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::ModifySubApiKey.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Delete the api key of master account. Use the api key pending to be delete to call the endpoint. Use master user's api key only.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/rm-master-apikey
    async fn delete_master_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::DeleteMasterApiKey.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }

    /// Delete the api key of sub account. Use the api key pending to be delete to call the endpoint. Use sub user's api key only.

    ///     Returns:
    ///         Request results as HashMap.

    ///     Additional information:
    ///         https://bybit-exchange.github.io/docs/v5/user/rm-sub-apikey
    async fn delete_sub_api_key(
        &self,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let endpoint = v5user::User::DeleteSubApiKey.to_string();
        let result = self
            .http_manager
            .submit_post_request(Method::POST, &endpoint, true, query)
            .await?;
        Ok(result)
    }
}

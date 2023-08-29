#![allow(unused)]
use async_trait::async_trait;
use http::method;
use reqwest::{header, Method};
use ring::hmac;
use serde_json::{json, to_string, Value};
use std::{
    collections::{BTreeMap, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};
use url::form_urlencoded::{self, Serializer};

use sha2::Sha256;

use crate::helpers::utils;

#[async_trait]
pub trait Manager {
    async fn auth(
        &self,
        req_params: &BTreeMap<String, String>,
        recv_window: u64,
        timestamp: u128,
    ) -> Result<String, String>;
    async fn submit_request(
        &self,
        method: Method,
        path: &str,
        query: HashMap<String, String>,
        auth: bool,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    async fn sign(
        &self,
        method: Method,
        path: &str,
        query: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        self.submit_request(method, path, query, true).await
    }

    async fn submit_post_request(
        &self,
        method: Method,
        path: &str,
        auth: bool,
        json_input: HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}
pub struct HttpManager {
    pub api_key: String,
    api_secret: String,
    base_url: String,
    recv_window: u64,
    ignore_codes: Vec<u64>,
    max_retries: u64,
    client: reqwest::Client,
}

impl HttpManager {
    ///
    ///
    /// Initializes a new HttpManager instance.
    ///
    ///
    pub fn new(api_key: String, api_secret: String, testnet: bool) -> Self {
        let sub_domain = if testnet { "api-testnet" } else { "api" };
        let domain = if testnet { "bybit" } else { "bybit" };
        let url = format!("https://{}.{}.com", sub_domain, domain);
        let client = reqwest::Client::new();

        HttpManager {
            api_key,
            api_secret,
            base_url: url,
            recv_window: 5000,
            ignore_codes: vec![],
            max_retries: 10,
            client,
        }
    }

    ///
    ///
    /// Generates authentication signature
    ///
    ///
    pub async fn generate_signature(
        &self,
        secret: &str,
        msg: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        let tag = hmac::sign(&key, msg.as_bytes());
        Ok(hex::encode(tag.as_ref()))
    }
}
#[async_trait]
impl Manager for HttpManager {
    ///
    ///
    /// Generates authentication signature per Bybit API specifications
    ///
    ///
    async fn auth(
        &self,
        req_params: &BTreeMap<String, String>,
        recv_window: u64,
        timestamp: u128,
    ) -> Result<String, String> {
        if self.api_key.is_empty() || self.api_secret.is_empty() {
            return Err("Authenticated endpoints require keys.".to_string());
        }
        let param_string =
            serde_urlencoded::to_string(req_params).map_err(|e| format!("Error: {:?}", e))?;

        let val = format!(
            "{time}{api_key}{recv_window}{params}",
            time = timestamp,
            api_key = self.api_key,
            recv_window = recv_window,
            params = param_string.to_string(),
        );
        let sign_result = self.generate_signature(&self.api_secret, &val).await;
        let sign = sign_result.map_err(|e| format!("Error: {:?}", e))?;
        Ok(format!("{}&sign={}", param_string, sign))
    }

    ///
    ///
    /// Submits the request to the API.
    /// Notes
    /// -------------------
    /// We use the params argument for the GET method, and data argument for
    /// the POST method. Dicts passed to the data argument must be
    /// JSONified prior to submitting request.
    /// -------------------
    ///
    async fn submit_request(
        &self,
        method: Method,
        path: &str,
        parameters: HashMap<String, String>,
        auth: bool,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let request_url = format!("{}{}", self.base_url, path);

        let mut request_builder = self.client.request(method.clone(), &request_url);

        if auth {
            let timestamp = utils::generate_timestamp()? as u128;
            let param_string = serde_urlencoded::to_string(&parameters)?;
            let val = format!(
                "{time}{api_key}{recv_window}{params}",
                time = timestamp,
                api_key = self.api_key,
                recv_window = self.recv_window,
                params = param_string,
            );

            let signature = self
                .generate_signature(&self.api_secret, &val)
                .await
                .map_err(|e| format!("Error: {:?}", e))?;

            let headers = utils::build_private_headers(
                &self.api_key,
                &signature,
                timestamp,
                &self.recv_window.to_string(),
            );
            request_builder = request_builder.headers(headers);
        }

        let response = match method {
            Method::GET | Method::DELETE => request_builder
                .query(&parameters)
                .send()
                .await
                .map_err(|e| format!("Error: {:?}", e))?,
            Method::POST | Method::PUT => {
                request_builder = request_builder.header(header::CONTENT_TYPE, "application/json");
                request_builder.json(&parameters).send().await?
            }
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unsupported HTTP method",
                )));
            }
        };

        let body_text = response.text().await?;
        let body: Value = serde_json::from_str(&body_text)?;

        Ok(body)
    }

    async fn submit_post_request(
        &self,
        method: Method,
        path: &str,
        auth: bool,
        json_input:  HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = utils::generate_timestamp()? as u128;
    
        let json_string = serde_json::to_string(&json_input)?;  // Convert the HashMap into a JSON string.
    
        let val = format!(
            "{time}{api_key}{recv_window}{params}",
            time = timestamp,
            api_key = self.api_key,
            recv_window = self.recv_window,
            params = json_string,
        );
    
        let signature = self
            .generate_signature(&self.api_secret, &val)
            .await
            .map_err(|e| format!("Error: {:?}", e))?;
    
        let request_url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .post(&request_url)
            .json(&json_input)  // Pass a reference to the HashMap.
            .headers(utils::build_private_headers(
                &self.api_key,
                &signature,
                timestamp,
                &self.recv_window.to_string(),
            ))
            .send()
            .await?;
    
        let body_text = response.text().await?;
        let body: Value = serde_json::from_str(&body_text)?;
    
        Ok(body)
    }
    
}

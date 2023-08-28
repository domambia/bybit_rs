#![allow(unused)]
use async_trait::async_trait;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{header, Method, Url};
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

    async fn auth_get(
        &self,
        req_params: &BTreeMap<String, String>,
        recv_window: u64,
        timestamp: u128,
    ) -> Result<String, String>;
    fn prepare_payload(
        &self,
        method: &str,
        parameters: HashMap<String, String>,
    ) -> BTreeMap<String, String>;
    async fn submit_request(
        &self,
        method: Method,
        path: &str,
        query: HashMap<String, String>,
        auth: bool,
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
        _recv_window: u64,
        _timestamp: u128,
    ) -> Result<String, String> {
        let api_key = &self.api_key;
        let api_secret = &self.api_secret;

        if api_key.is_empty() || api_secret.is_empty() {
            return Err("Authenticated endpoints require keys.".to_string());
        }

        let mut query = Serializer::new(String::new());
        for (key, value) in req_params.iter() {
            query.append_pair(key, value);
        }

        let param_string = query.finish();
        let sign = utils::sign_query_string(&param_string, &api_secret)
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(format!("{}&sign={}", param_string, sign))
    }

    async fn auth_get(
        &self,
        req_params: &BTreeMap<String, String>,
        _recv_window: u64,
        _timestamp: u128,
    ) -> Result<String, String> {
        let api_key = &self.api_key;
        let api_secret = &self.api_secret;

        if api_key.is_empty() || api_secret.is_empty() {
            return Err("Authenticated endpoints require keys.".to_string());
        }

        let params: Vec<(String, String)> = req_params
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        println!("Params: {:?}", params);
        // correct

        let query_string =
            serde_urlencoded::to_string(params).map_err(|e| format!("Error: {:?}", e))?;
        // correct

        println!("Query String: {:?}", query_string);
        let val = format!(
            "{time}{api_key}{recv}{params}",
            time = _timestamp,
            api_key = &api_key,
            recv = _recv_window,
            params = query_string,
        );

        println!("VAL: {}", val);

        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &api_secret.as_bytes());
        let tag = ring::hmac::sign(&key, val.as_bytes());
        println!("TAG: {:?}", hex::encode(tag.as_ref()));

        Ok(hex::encode(tag.as_ref()))
    }

    ///
    /// Prepares payload and submit request
    /// GET requests are formatted as query strings
    /// POST requests are formatted as JSON
    ///
    fn prepare_payload(
        &self,
        method: &str,
        mut parameters: HashMap<String, String>,
    ) -> BTreeMap<String, String> {
        fn cast_values(parameters: &mut HashMap<String, String>) {
            let string_params = ["qty", "price", "triggerPrice", "takeProfit", "stopLoss"];
            let integer_params = ["positionIdx"];

            for (key, value) in parameters.iter_mut() {
                if string_params.contains(&key.as_str()) {
                    if value.parse::<i64>().is_err() {
                        // Handle parsing error, or simply leave value unchanged
                    }
                } else if integer_params.contains(&key.as_str()) {
                    if !value.is_empty() {
                        let parsed_value = value.parse::<i64>();
                        if let Ok(parsed) = parsed_value {
                            *value = parsed.to_string();
                        }
                    }
                }
            }
        }

        if method == "GET" {
            let mut payload = BTreeMap::new();
            for (k, v) in parameters.iter() {
                if !v.is_empty() {
                    payload.insert(k.clone(), v.clone());
                }
            }
            payload
        } else {
            cast_values(&mut parameters);
            parameters.into_iter().collect()
        }
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
        query: HashMap<String, String>,
        auth: bool,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let mut req_params: BTreeMap<String, String> = BTreeMap::new();
        let mut recv_window = self.recv_window;
        let new_method = method.clone().to_string();

        if auth {
            let timestamp = utils::generate_timestamp()?;
            recv_window = std::cmp::min(recv_window, self.recv_window);

            let mut new_query: HashMap<String, String> = query.clone();
            new_query.insert("timestamp".to_string(), timestamp.to_string());
            new_query.insert("api_key".to_string(), self.api_key.to_string());
            req_params = self.prepare_payload(method.as_str(), new_query.clone());
            // signature for post,patch & put method
            let sign: String = self.auth(&req_params, recv_window, timestamp).await?;

            let mut url = format!("{}{}?{}", self.base_url, path, &sign);

            let response = match method {
                Method::GET => {
                    let sign_get: String = self
                        .auth_get(
                            &self.prepare_payload(method.as_str(), query.clone()),
                            recv_window,
                            timestamp,
                        )
                        .await?;

                    println!("sign_get: {}", sign_get);

                    //  build private headers
                    let mut headers = utils::build_private_headers(
                        &sign_get,
                        timestamp,
                        &self.api_key,
                        &recv_window.to_string(),
                    );

                    println!("Headers: {:?}", headers);

                    // generate GET query params
                    let query_params = serde_urlencoded::to_string(query.clone())?;
                    let url = format!(
                        "{}{}?{}",
                        self.base_url,
                        path,
                        if new_method == "GET" {
                            &query_params
                        } else {
                            &sign
                        }
                    );

                    self.client.get(&url).headers(headers.clone()).send().await
                }
                Method::POST => self.client.post(&url).send().await,
                Method::PUT => self.client.put(&url).send().await,
                Method::DELETE => self.client.delete(&url).send().await,
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unsupported HTTP method",
                    )))
                }
            };

            match response {
                Ok(response) => {
                    let body_text = response.text().await?;
                    let body: serde_json::Value = serde_json::from_str(&body_text)?;
                    let resp_data = serde_json::to_string_pretty(&body)?;

                    return Ok((resp_data).parse::<Value>().unwrap());
                }
                Err(e) => println!("Request failed: {}", e),
            }
        }

        Ok(Value::Null)
    }
}

#![allow(unused)]
use async_trait::async_trait;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{header, Method, Url};
use serde_json::{json, to_string, Value};
use std::{
    collections::{BTreeMap, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};
use url::form_urlencoded;

use sha2::Sha256;

use crate::helpers::utils;

#[async_trait]
pub trait Manager {
    fn auth(
        &self,
        req_params: &BTreeMap<String, String>,
        recv_window: u64,
        timestamp: u64,
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
    fn auth(
        &self,
        req_params: &BTreeMap<String, String>,
        _recv_window: u64, // I noticed recv_window and timestamp aren't used in this function
        _timestamp: u64,
    ) -> Result<String, String> {
        let api_key = &self.api_key;
        let api_secret = &self.api_secret;

        if api_key.is_empty() || api_secret.is_empty() {
            return Err("Authenticated endpoints require keys.".to_string());
        }

        // Convert AppError to String
        let signature = utils::gen_query_string_with_singature(&req_params, &api_secret)
            .map_err(|e| format!("Error: {:?}", e))?;

        Ok(signature)
    }

    // fn prepare_payload(
    //     &self,
    //     method: &str,
    //     parameters: HashMap<String, String>,
    // ) -> BTreeMap<String, String> {
    //     if method == "GET" {
    //         parameters.into_iter().collect()
    //     } else {
    //         parameters.into_iter().collect()
    //     }
    // }

    ///
    /// Generate timestamp
    ///

    ///
    /// Prepares payload for request
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

        if auth {
            let timestamp = utils::generate_timestamp();
            recv_window = std::cmp::min(recv_window, self.recv_window);

            let mut new_query: HashMap<String, String> = query.clone();
            new_query.insert("timestamp".to_string(), timestamp.to_string());
            new_query.insert("api_key".to_string(), self.api_key.to_string());
            req_params = self.prepare_payload(method.as_str(), new_query.clone());
            let post_put_update_singature = self.auth(&req_params, recv_window, timestamp)?;

            // let get_sing = utils::sign_for_query(&req_params, &self.api_secret)
            //     .map_err(|e| format!("Error: {:?}", e))?;

            let mut headers = header::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "application/json".parse()?);
            headers.insert("X-BAPI-API-KEY", self.api_key.parse()?);
            headers.insert("X-BAPI-SIGN", post_put_update_singature.parse()?);
            headers.insert("X-BAPI-TIMESTAMP", timestamp.to_string().parse()?);
            headers.insert("X-BAPI-RECV-WINDOW", recv_window.to_string().parse()?);
            let get_query_string = utils::generate_query_data(new_query.clone());

            let post_put_path_url =
                format!("{}{}?{}", self.base_url, path, post_put_update_singature);
            let get_url = format!("{}{}?{}", self.base_url, path, get_query_string);

            // println!("Query string: {:?}", get_url);
            // println!("headers: {:?}", headers);

            let response = match method {
                Method::GET => self.client.get(&get_url).headers(headers).send().await,
                Method::POST => self.client.post(&post_put_path_url).send().await,
                Method::PUT => self.client.put(&post_put_path_url).send().await,
                Method::DELETE => self.client.delete(&post_put_path_url).send().await,
                // Add more cases for other HTTP methods
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unsupported HTTP method",
                    )))
                }
            };

            match response {
                Ok(response) => {
                    let body_text = response.text().await?; // Corrected placement of await
                    let body: serde_json::Value = serde_json::from_str(&body_text)?; // Corrected placement of await
                    let resp_data = serde_json::to_string_pretty(&body)?;

                    return Ok((resp_data).parse::<Value>().unwrap());
                }
                Err(e) => println!("Request failed: {}", e),
            }
        }

        Ok(Value::Null)
    }
}

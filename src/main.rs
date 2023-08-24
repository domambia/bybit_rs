#![allow(unused)]
pub mod bybit;
use std::collections::HashMap;

use bybit::http_manager::{HttpManager, Manager};

#[tokio::main]
async fn main() {
    let http_api_key = "bWGzHP5jDyV2nv9jfO";
    let http_api_secret = "VQMrhV1r8nNCIe4zLuPdXoUIyi6jSrsWtbSs";
    let testnet = true;

    let manager = HttpManager::new(
        http_api_key.to_string(),
        http_api_secret.to_string(),
        testnet,
    );

    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_string(), "inverse".to_string());
    query.insert("symbol".to_string(), "BTCUSD".to_string());
    query.insert("interval".to_string(), "60".to_string());

    match manager
        .submit_request(reqwest::Method::GET, "/v5/market/kline", query, true)
        .await
    {
        Ok(result) => println!("{:?}", result["result"].clone()),
        Err(e) => println!("{:?}", e),
    };
}

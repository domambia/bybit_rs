#![allow(unused)]
use std::collections::HashMap;
pub mod bybit;
pub mod endpoints;
pub mod helpers;

use bybit::{
    http_manager::{HttpManager, Manager},
    trade::{self, Trade},
};

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

    // let mut query: HashMap<String, String> = HashMap::new();
    // query.insert("category".to_string(), "inverse".to_string());
    // query.insert("symbol".to_string(), "BTCUSD".to_string());
    // query.insert("interval".to_string(), "60".to_string());

    // match manager
    //     .submit_request(reqwest::Method::GET, "/v5/market/kline", query, true)
    //     .await
    // {
    //     Ok(result) => println!("{:?}", result["result"].clone()),
    //     Err(e) => println!("{:?}", e),
    // };

    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_string(), "spot".to_string());
    query.insert("symbol".to_string(), "BTCUSDT".to_string());
    query.insert("side".to_string(), "Buy".to_string());
    query.insert("orderType".to_string(), "Market".to_string());
    query.insert("qty".to_string(), "200".to_string());
    query.insert("orderFilter".to_string(), "Order".to_string());

    let trade: trade::TradeHTTP = trade::TradeHTTP::new(manager);

    match trade.place_order(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    };
}

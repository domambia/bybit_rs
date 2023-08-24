use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_timestamp() -> u128 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    timestamp
}

// fn identify_ws_method<'a>(
//     input_wss_url: &'a str,
//     wss_dictionary: &'a HashMap<&'a str, &'a str>,
// ) -> Option<&'a str> {
//     let path = regex::Regex::new(r"(wss://)?([^/\s]+)(.*)").unwrap();
//     let input_wss_url_path = path.captures(input_wss_url).unwrap()[3];
//     for (wss_url, function_call) in wss_dictionary {
//         let wss_url_path = path.captures(wss_url).unwrap()[3];
//         if input_wss_url_path == wss_url_path {
//             return Some(function_call);
//         }
//     }
//     None
// }

fn find_index(
    source: &Vec<HashMap<&str, &str>>,
    target: &HashMap<&str, &str>,
    key: &str,
) -> Option<usize> {
    source.iter().position(|item| item[key] == target[key])
}

// fn are_connections_connected(active_connections: &Vec<&ConnectionType>) -> bool {
//     active_connections
//         .iter()
//         .all(|connection| connection.is_connected())
// }

fn is_inverse_contract(symbol: &str) -> bool {
    if regex::Regex::new(r"(USD)([HMUZ]\d\d|$)")
        .unwrap()
        .is_match(symbol)
    {
        true
    } else {
        false
    }
}

fn is_usdt_perpetual(symbol: &str) -> bool {
    symbol.ends_with("USDT")
}

fn is_usdc_perpetual(symbol: &str) -> bool {
    symbol.ends_with("USDC")
}

fn is_usdc_option(symbol: &str) -> bool {
    if regex::Regex::new(r"[A-Z]{3}-.*-[PC]$")
        .unwrap()
        .is_match(symbol)
    {
        true
    } else {
        false
    }
}

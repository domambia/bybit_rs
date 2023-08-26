use hmac::{Hmac, Mac, NewMac};
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::sign::Signer;
use sha2::Sha256;
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded::{self, Serializer};

use crate::errors::app_error::AppError;

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

pub fn gen_query_string_with_singature(
    params: &BTreeMap<String, String>,
    secret: &str,
) -> Result<String, AppError> {
    let mut query = Serializer::new(String::new());
    for (key, value) in params.iter() {
        query.append_pair(key, value);
    }

    let param_string = query.finish();
    let sign = sign_query_string(&param_string, secret)?;
    Ok(format!("{}&sign={}", param_string, sign))
}

pub fn gen_query_string(
    params: &BTreeMap<String, String>,
    secret: &str,
) -> Result<String, AppError> {
    let mut query = Serializer::new(String::new());
    for (key, value) in params.iter() {
        query.append_pair(key, value);
    }

    let param_string = query.finish();
    let sign = sign_query_string(&param_string, secret)?;
    Ok(format!("{}", sign))
}

pub fn generate_query_data(params: HashMap<String, String>) -> String {
    let mut query = Serializer::new(String::new());
    for (key, value) in params.iter() {
        query.append_pair(key, value);
    }
    query.finish()
}

fn sign_query_string(query_string: &str, secret: &str) -> Result<String, AppError> {
    let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).map_err(|_| AppError::HmacError)?;
    mac.update(query_string.as_bytes());
    Ok(bytes_to_hex(mac.finalize().into_bytes().to_vec()))
}

fn bytes_to_hex(bytes: Vec<u8>) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

// pub fn extract_value<'a>(input: &'a str, key: &'a str) -> Option<&'a str> {
//     let key_str = format!("{}=", key);
//     if let Some(start) = input.find(&key_str) {
//         let end = input[start + key_str.len()..]
//             .find('&')
//             .unwrap_or(input.len());
//         Some(&input[start + key_str.len()..start + key_str.len() + end])
//     } else {
//         None
//     }
// }

pub fn remove_key(input: &str, key: &str) -> String {
    let key_str = format!("{}=", key);
    if let Some(start) = input.find(&key_str) {
        let end = input[start..].find('&').unwrap_or(input.len());
        let before = &input[..start];
        let after = &input[start + key_str.len() + end..];
        format!("{}{}", before, after)
    } else {
        input.to_string()
    }
}

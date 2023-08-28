use hmac::{Hmac, Mac, NewMac};
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::sign::Signer;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use sha2::Sha256;
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded::{self, Serializer};

use crate::errors::app_error::AppError;

///
///Generate custom timestamp in milliseconds
///
pub fn generate_timestamp() -> Result<u128, std::time::SystemTimeError> {
    let d = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(d.as_millis())
}

///
///
///
///
pub fn sign_query_string(query_string: &str, secret: &str) -> Result<String, AppError> {
    let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).map_err(|_| AppError::HmacError)?;
    mac.update(query_string.as_bytes());
    Ok(bytes_to_hex(mac.finalize().into_bytes().to_vec()))
}

///
///
///
fn bytes_to_hex(bytes: Vec<u8>) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

///
///
/// Build custom headers
/// Params
///     signature
///     timestamp :  u64
///     api_key: &str
///     recv_window:  &str
/// Returns:  HeaderMap
///
///
pub fn build_private_headers(
    signature: &str,
    timestamp: u128,
    api_key: &str,
    recv_window: &str,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&api_key).unwrap());
    headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap());
    headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_static("2"));
    headers.insert(
        "X-BAPI-TIMESTAMP",
        HeaderValue::from_str(&timestamp.to_string()).expect("Invalid timestamp"),
    );
    headers.insert(
        "X-BAPI-RECV-WINDOW",
        HeaderValue::from_str(&recv_window).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

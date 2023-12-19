pub mod account;
pub mod asset;
pub mod broker;
pub mod http_manager;
pub mod market;
pub mod position;
pub mod spot_leverage_token;
pub mod spot_margin_trade;
pub mod trade;
pub mod user;


type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

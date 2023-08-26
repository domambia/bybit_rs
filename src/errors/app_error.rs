use std::fmt;

#[derive(Debug)]
pub enum AppError {
    EnvVarMissing(String),
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    HmacError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::EnvVarMissing(var_name) => {
                write!(f, "{} environment variable not set", var_name)
            }
            AppError::RequestError(err) => write!(f, "Request error: {}", err),
            AppError::JsonError(err) => write!(f, "JSON error: {}", err),
            AppError::HmacError => write!(f, "HMAC creation error"),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::RequestError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err)
    }
}

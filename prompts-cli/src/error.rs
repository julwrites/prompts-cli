use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("JSON serialization/deserialization error: {0}")]
    Json(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("An unexpected error occurred: {0}")]
    Anyhow(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err.to_string())
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::Config(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err.to_string())
    }
}

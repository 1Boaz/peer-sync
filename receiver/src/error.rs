use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiddlewareError {
    #[error("Missing or invalid Authorization header")]
    Unauthorized,

    #[error("Server configuration error: {0}")]
    Configuration(String),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("File operation failed: {0}")]
    FileOperation(String),

    #[error("Invalid request: {0}")]
    #[allow(dead_code)]
    InvalidRequest(String),

    #[error("Middleware error: {0}")]
    Middleware(#[from] MiddlewareError),
}

pub type Result<T> = std::result::Result<T, AppError>;
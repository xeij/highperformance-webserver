use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
    
    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Route not found: {method} {path}")]
    RouteNotFound { method: String, path: String },
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ServerError {
    pub fn status_code(&self) -> hyper::StatusCode {
        match self {
            ServerError::RouteNotFound { .. } => hyper::StatusCode::NOT_FOUND,
            ServerError::BadRequest(_) => hyper::StatusCode::BAD_REQUEST,
            _ => hyper::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
} 
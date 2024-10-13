use thiserror::Error;

/// Errors that can occur during Shindan operations.
#[derive(Error, Debug)]
pub enum ShindanError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("HTML parsing failed: {0}")]
    Parse(String),

    #[error("Required form token not found: {0}")]
    TokenNotFound(&'static str),

    #[error("Session cookie not found")]
    SessionCookieNotFound,
}

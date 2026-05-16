use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefrainError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("rewrite error: {0}")]
    Rewrite(String),
    #[error("adapter error: {0}")]
    Adapter(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RefrainError>;

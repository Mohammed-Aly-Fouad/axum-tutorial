use thiserror::Error;
// use tokio::time::error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Failed datatbase query: {0}")]
    Query(#[from] sqlx::Error),
    
    #[error("Failed to query: {0}")]
    FailedQuery(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Failed to hash: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}
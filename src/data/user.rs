use sqlx::PgPool;
use super::errors::DataError;

// use bcrypt;
// use tokio::time::error::Error;
// use crate::{handlers::todos::todos_handler};

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {

    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    let bytea_hash = hashed_password.as_bytes();

    // let query: &str = r#"INSERT INTO users(email, password_hash)
    // VALUES ($1, $2)"#;

    // sqlx::query(&query).bind(email).bind(bytea_hash).execute(pool).await?;

    sqlx::query!("INSERT INTO clients(email, password_hash)
    VALUES($1, $2)",
    email,
    bytea_hash
    )
    .execute(pool)
    .await.map_err(|err| {
        match err {
            sqlx::Error::Database(e) => {
                if e.constraint() == Some("users_email_key") {
                    DataError::FailedQuery("This email address is already used".to_string())
                } else {
                    DataError::Internal(e.to_string())
                }
            },
            e => DataError::Query(e),
            }

    })?;

    Ok(())

    // Err(DataError::Internal("Test error".to_string()))

}

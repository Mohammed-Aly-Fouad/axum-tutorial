use bcrypt;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    let bytea_hash = hashed_password.as_bytes();

    let query: &str = r#"INSERT INTO users(email, password_hash)
    VALUES ($1, $2)"#;

    // sqlx::query(&query).bind(email).bind(bytea_hash).execute(pool).await?;

    sqlx::query!("INSERT INTO users(email, password_hash)
    VALUES($1, $2)", email, bytea_hash).execute(pool).await?;

    Ok(())

}
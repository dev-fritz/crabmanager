use sqlx::mysql::{MySqlPoolOptions, MySqlPool};

pub async fn establish_connection(database_url: String) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .idle_timeout(std::time::Duration::from_secs(10))
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
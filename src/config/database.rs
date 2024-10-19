use sqlx::mysql::{MySqlPoolOptions, MySqlPool};


/// Establish connection to the database
pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost:3306/db_name").await;
    pool
}


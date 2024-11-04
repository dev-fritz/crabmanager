use log::error;
use sqlx::MySqlPool;

use crate::models::user_model::{UserData, UserLogin, UserRegister};

pub trait UserRepository {
    async fn get_user_by_email(&self, conn: MySqlPool, email: &str) -> Result<UserData, String>;
    async fn get_user_by_id(&self, conn: MySqlPool, id: i32) -> Result<UserData, String>;
    async fn register_user(&self, conn: MySqlPool, user: UserRegister);
    async fn login_user(&self, conn: MySqlPool, user: UserLogin) -> Result<UserData, String>;
}

// Define a struct that implements the UserRepository trait
pub struct DefaultUserRepository {
    // Add any necessary fields here, for example:
    // users: HashMap<String, UserModel>,
}

// Implement the UserRepository trait for the DefaultUserRepository struct
impl UserRepository for DefaultUserRepository {
    async fn get_user_by_email(&self, conn: MySqlPool, email: &str) -> Result<UserData, String> {
        let user = sqlx::query_as::<_, UserData>(
            r#"
            SELECT id, name, email, phone, password, created_at, updated_at, user_type, status, merchant_id
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_one(&conn)
        .await
        .map_err(|e| {
            error!("Failed to get user by email: {}", e);
            "Failed to get user by email".to_string()
        })?;
        Ok(UserData {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            created_at: user.created_at,
            user_type: user.user_type,
            status: user.status,
            merchant_id: user.merchant_id,
        })
    }

    async fn get_user_by_id(&self, conn: MySqlPool, id: i32) -> Result<UserData, String> {
        let user = sqlx::query_as::<_, UserData>(
            r#"
            SELECT id, name, email, phone, password, created_at, updated_at, user_type, status, merchant_id
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&conn)
        .await
        .map_err(|e| {
            error!("Failed to get user by id: {}", e);
            "Failed to get user by id".to_string()
        })?;
        Ok(UserData {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            created_at: user.created_at,
            user_type: user.user_type,
            status: user.status,
            merchant_id: user.merchant_id,
        })
    }

    async fn register_user(&self, conn: MySqlPool, user: UserRegister) {
        sqlx::query_as::<_, UserData>(
            r#"
            INSERT INTO users (name, email, phone, password, user_type, status)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.password)
        .bind(&user.user_type)
        .bind(&user.status)
        .fetch_optional(&conn)
        .await
        .expect("Failed to insert user into database");
    }

    async fn login_user(&self, conn: MySqlPool, user: UserLogin) -> Result<UserData, String> {
        let user = sqlx::query_as::<_, UserData>(
            r#"
            SELECT id, name, email, phone, created_at, user_type, status, merchant_id
            FROM users
            WHERE email = ? AND password = ?
            "#,
        )
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&conn)
        .await
        .map_err(|e| {
            error!("Failed to get user by email: {}", e);
            "Failed to get user by email".to_string()
        })?;
        Ok(UserData {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            created_at: user.created_at,
            user_type: user.user_type,
            status: user.status,
            merchant_id: user.merchant_id,
        })
    }
}

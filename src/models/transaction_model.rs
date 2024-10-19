use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionModel {
    pub id: i32,
    pub user_id: Option<i32>,
    pub merchant_id: i32,
    pub order_id: i32,
    pub amount: f64,
    pub payment_method: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub user_id: Option<i32>,
    pub merchant_id: i32,
    pub order_id: i32,
    pub amount: f64,
    pub payment_method: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
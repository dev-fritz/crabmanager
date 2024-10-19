use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemModel {
    pub id: i32,
    pub order_id: i32,
    pub discount: Option<f64>,
    pub price: Option<f64>,
    pub product_id: i32,
    pub quantity: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub merchant_id: i32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderItem {
    pub order_id: i32,
    pub discount: Option<f64>,
    pub price: Option<f64>,
    pub product_id: i32,
    pub quantity: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub merchant_id: i32,
    pub status: String,
}
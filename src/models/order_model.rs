use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use super::order_item_model::OrderItemModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModel {
    pub id: i32,
    pub user_id: Option<i32>,
    pub merchant_id: i32,
    pub total_price: f64,
    pub total_quantity: f64,
    pub discount: f64,
    pub order_type: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub order_items: Vec<OrderItemModel>,
    pub transaction_id: Option<String>,
    pub payment_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub user_id: Option<i32>,
    pub merchant_id: i32,
    pub total_price: f64,
    pub total_quantity: f64,
    pub discount: f64,
    pub order_type: String,
    pub status: String,
    pub order_items: Vec<OrderItemModel>,
    pub transaction_id: Option<String>,
    pub payment_status: String,
}
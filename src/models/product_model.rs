use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub merchant_id: i32,
    pub product_category_id: i32,
    pub stock: f64,
    pub status: String,
    pub image: Option<String>,
    pub ean: Option<String>,
    pub product_type: String,
    pub unit: String,
    pub weight: Option<f64>,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantModel {
    pub id: i32,
    pub name: String,
    pub responsible_user: i32,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub merchant_type: String,
    pub address: Option<String>,
    pub status: String,
    pub slug: String,
}
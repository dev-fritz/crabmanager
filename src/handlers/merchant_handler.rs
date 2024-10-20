use actix_web::{get, post, put, web, HttpResponse};

use crate::models::merchant_model::CreateMerchant;

#[get("/")]
async fn get_merchants() -> HttpResponse {
    HttpResponse::Ok().body("Get all merchants")
}

#[get("/{id}")]
async fn get_merchant_details(merchant_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get merchant")
}

#[post("/")]
async fn create_merchant(merchant: web::Json<CreateMerchant>) -> HttpResponse {
    todo!("Implement create merchant")
}

#[put("/{id}")]
async fn update_merchant(merchant_id: web::Path<i32>, merchant: web::Json<CreateMerchant>) -> HttpResponse {
    todo!("Implement update merchant, and change the CreateMerchant to UpdateMerchant");
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/merchant")
            .service(get_merchant_details)
            .service(get_merchants)
            .service(create_merchant)
            .service(update_merchant)
    );
}
use actix_web::{get, post, put, web, HttpResponse};

use crate::models::product_model::{ProductModel, CreateProduct};

#[get("/")]
async fn get_products() -> HttpResponse {
    HttpResponse::Ok().body("Get all products")
}

#[get("/{id}")]
async fn get_product_details(product_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get product")
}

#[post("/")]
async fn create_product(product: web::Json<CreateProduct>) -> HttpResponse {
    todo!("Implement create product")
}

#[put("/{id}")]
async fn update_product(product_id: web::Path<i32>, product: web::Json<CreateProduct>) -> HttpResponse {
    todo!("Implement update product, and change the CreateProduct to UpdateProduct");
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .service(get_product_details)
            .service(get_products)
            .service(create_product)
            .service(update_product)
    );
}
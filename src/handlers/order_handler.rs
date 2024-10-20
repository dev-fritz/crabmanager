use actix_web::{get, post, put, web, HttpResponse};

use crate::models::order_model::CreateOrder;

#[get("/")]
async fn get_orders() -> HttpResponse {
    HttpResponse::Ok().body("Get all orders")
}

#[get("/{id}")]
async fn get_order_details(order_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get order")
}

#[post("/")]
async fn create_order(order: web::Json<CreateOrder>) -> HttpResponse {
    todo!("Implement create order")
}

#[put("/{id}")]
async fn update_order(order_id: web::Path<i32>, order: web::Json<CreateOrder>) -> HttpResponse {
    todo!("Implement update order, and change the CreateOrder to UpdateOrder");
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .service(get_order_details)
            .service(get_orders)
            .service(create_order)
            .service(update_order)
    );
}
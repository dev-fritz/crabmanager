use actix_web::{get, post, put, web, HttpResponse};

use crate::models::order_item_model::{OrderItemModel, CreateOrderItem};

#[get("/")]
async fn get_order_items() -> HttpResponse {
    HttpResponse::Ok().body("Get all order_items")
}

#[get("/{id}")]
async fn get_order_item_details(order_item_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get order_item")
}

#[post("/")]
async fn create_order_item(order_item: web::Json<CreateOrderItem>) -> HttpResponse {
    todo!("Implement create order_item")
}

#[put("/{id}")]
async fn update_order_item(order_item_id: web::Path<i32>, order_item: web::Json<CreateOrderItem>) -> HttpResponse {
    todo!("Implement update order_item, and change the CreateOrderItem to UpdateOrderItem");
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order-item")
            .service(get_order_item_details)
            .service(get_order_items)
            .service(create_order_item)
            .service(update_order_item)
    );
}
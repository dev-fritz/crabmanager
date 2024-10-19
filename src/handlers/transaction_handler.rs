use actix_web::{get, post, put, web, HttpResponse};

use crate::models::transaction_model::{TransactionModel, CreateTransaction};

#[get("/")]
async fn get_transactions() -> HttpResponse {
    HttpResponse::Ok().body("Get all transactions")
}

#[get("/{id}")]
async fn get_transaction_details(transaction_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get transaction")
}

#[post("/")]
async fn create_transaction(transaction: web::Json<CreateTransaction>) -> HttpResponse {
    todo!("Implement create transaction")
}

#[put("/{id}")]
async fn update_transaction(transaction_id: web::Path<i32>, transaction: web::Json<CreateTransaction>) -> HttpResponse {
    todo!("Implement update transaction, and change the CreateTransaction to UpdateTransaction");
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transaction")
            .service(get_transaction_details)
            .service(get_transactions)
            .service(create_transaction)
            .service(update_transaction)
    );
}
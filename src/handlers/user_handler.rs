use actix_web::{get, post, put, web, HttpResponse};

use crate::models::user_model::{UserRegister, UserLogin, UserModel};

#[get("/{merchant_id}")]
async fn get_users_by_merchant(merchant_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().body("Get all users")
}

#[get("/{id}")]
async fn get_user(user_id: web::Path<i32>) -> HttpResponse {
    todo!("Implement get user")
}

#[post("/")]
async fn create_user(user: web::Json<UserRegister>) -> HttpResponse {
    todo!("Implement create user")
}

#[put("/{id}")]
async fn update_user(user_id: web::Path<i32>, user: web::Json<UserModel>) -> HttpResponse {
    todo!("Implement update user")
}

#[post("/")]
async fn login(user: web::Json<UserLogin>) -> HttpResponse {
    todo!("Implement login")
}

#[put("/reset-password")]
async fn reset_password() -> HttpResponse {
    todo!("Implement reset password")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_user)
            .service(get_users_by_merchant)
            .service(create_user)
            .service(update_user)
            .service(login)
            .service(reset_password),
    );
}
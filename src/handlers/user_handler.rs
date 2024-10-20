use actix_web::{get, post, put, web, HttpResponse};

use crate::models::user_model::{UserRegister, UserLogin, UserModel};
use crate::repositories::user_repository::{DefaultUserRepository, UserRepository};
use crate::AppState; // Import AppState to access db conn

const USER_REPOSITORY: DefaultUserRepository = DefaultUserRepository {};

#[get("/{email}")]
async fn get_users_by_merchant(email: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let user = USER_REPOSITORY.get_user_by_email(state.db_pool.clone(), &email).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[get("/{id}")]
async fn get_user(user_id: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let user = USER_REPOSITORY.get_user_by_id(state.db_pool.clone(), *user_id).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/")]
async fn create_user(user_data: web::Json<UserRegister>, state: web::Data<AppState>) -> HttpResponse {
    let conn = state.db_pool.clone();

    USER_REPOSITORY.register_user(conn, user_data.into_inner()).await;

    HttpResponse::Created().body("User created successfully")
}

#[put("/{id}")]
async fn update_user(user_id: web::Path<i32>, user: web::Json<UserModel>, state: web::Data<AppState>) -> HttpResponse {
    let conn = state.db_pool.clone();

    todo!("Implement update user with db conn")
}

#[post("/")]
async fn login(user: web::Json<UserLogin>, state: web::Data<AppState>) -> HttpResponse {
    let conn = state.db_pool.clone();

    todo!("Implement login with db conn")
}

#[put("/reset-password")]
async fn reset_password(state: web::Data<AppState>) -> HttpResponse {
    let conn = state.db_pool.clone();

    todo!("Implement reset password with db conn")
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
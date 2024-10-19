mod models;
mod handlers;
mod services;
mod config;
mod repositories;

use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main () -> Result<(), std::io::Error> {
    
    HttpServer::new(|| {
        
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .service(index)
        
    })
        
    .bind(("127.0.0.1", 8080)).unwrap()
    .run()
    .await
}
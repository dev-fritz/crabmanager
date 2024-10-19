pub mod product_handler;
pub mod user_handler;
pub mod merchant_handler;
pub mod order_handler;
pub mod order_item_handler;
pub mod transaction_handler;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    product_handler::init_routes(cfg);
    user_handler::init_routes(cfg);
    merchant_handler::init_routes(cfg);
    order_handler::init_routes(cfg);
    order_item_handler::init_routes(cfg);
    transaction_handler::init_routes(cfg);
} 
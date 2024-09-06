use axum::{
    routing::{get, post},
    Router,
};
use create_order::create_order_handler;
use list_my_orders::list_my_orders_handler;

use crate::models::app_state::AppState;

mod create_order;
mod errors;
mod list_my_orders;

pub fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_order_handler))
        .route("/list-my", get(list_my_orders_handler))
}

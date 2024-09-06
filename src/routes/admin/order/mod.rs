use axum::{
    routing::{get, post},
    Router,
};
use get_attachment::get_attachment_handler;
use list_undelivered::list_undelivered_handler;
use mark_delivered::mark_delivered_handler;

use crate::models::app_state::AppState;

mod errors;
mod get_attachment;
mod list_undelivered;
mod mark_delivered;

pub fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/attachment/:order_id", get(get_attachment_handler))
        .route("/mark-delivered/:order_id", post(mark_delivered_handler))
        .route("/list-undelivered", get(list_undelivered_handler))
}

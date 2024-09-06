use axum::Router;
use order::order_routes;

use crate::models::app_state::AppState;

use self::{admin::admin_routes, auth::auth_routes, user::user_routes};

mod admin;
mod auth;
mod order;
mod user;

pub fn open_routes() -> Router<AppState> {
    Router::new().nest("/auth", auth_routes())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user_routes())
        .nest("/order", order_routes())
        .nest("/admin", admin_routes())
}

use axum::{routing::post, Router};

use crate::models::app_state::AppState;

use self::{login::login_handler, register::register_handler};

mod errors;
mod login;
mod register;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
}

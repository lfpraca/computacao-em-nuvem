use axum::{routing::post, Router};

use crate::models::app_state::AppState;

use self::logout::logout_handler;

mod logout;

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/logout", post(logout_handler))
}

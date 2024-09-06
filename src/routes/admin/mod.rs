use axum::{middleware::from_fn, Router};
use order::order_routes;

use crate::{
    middleware::allowed_roles::allowed_roles,
    models::{app_state::AppState, user_role::UserRole},
};

mod order;

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .nest("/order", order_routes())
        .layer(from_fn(|e, r, n| {
            allowed_roles(e, r, n, &[UserRole::Admin as i16])
        }))
}

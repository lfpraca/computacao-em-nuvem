use crate::{
    errors::db_error::DbError,
    models::{app_state::AppState, token_data::TokenData},
};
use axum::{debug_handler, extract::State, response::IntoResponse, Extension, Json};

#[debug_handler]
pub async fn list_my_orders_handler(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<TokenData>,
) -> Result<impl IntoResponse, DbError> {
    Ok(Json(
        app_state
            .db_context()
            .order()
            .list_recent_for_user(token_data.user_id(), 10)
            .await?,
    ))
}

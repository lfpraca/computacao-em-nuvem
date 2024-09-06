use axum::{debug_handler, extract::State, response::IntoResponse, Json};

use crate::{errors::db_error::DbError, models::app_state::AppState};

#[debug_handler]
pub async fn list_undelivered_handler(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, DbError> {
    Ok(Json(
        app_state.db_context().order().list_undelivered().await?,
    ))
}

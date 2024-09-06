use axum::{
    debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{errors::db_error::DbError, models::app_state::AppState};

#[debug_handler]
pub async fn mark_delivered_handler(
    State(app_state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<(), DbError> {
    app_state
        .db_context()
        .order()
        .mark_delivered(order_id)
        .await?;

    Ok(())
}

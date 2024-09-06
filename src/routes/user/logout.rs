use axum::{debug_handler, extract::State, Extension};

use crate::{
    errors::db_error::DbError,
    models::{app_state::AppState, token_data::TokenData},
};

#[debug_handler]
pub async fn logout_handler(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<TokenData>,
) -> Result<(), DbError> {
    app_state
        .db_context()
        .user_token()
        .delete_token(token_data.token())
        .await?;

    Ok(())
}

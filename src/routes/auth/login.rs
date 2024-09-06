use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{debug_handler, extract::State, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::models::app_state::AppState;

use super::errors::login_error::LoginError;

#[debug_handler]
pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<String, LoginError> {
    let (id, pass_hash) = app_state
        .db_context()
        .user()
        .fetch_login_data(&body.phone_number)
        .await?
        .ok_or(LoginError::NotFound)?;

    let pass_hash = PasswordHash::new(&pass_hash)?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &pass_hash)
        .map_err(|_| LoginError::InvalidPassword)?;

    let token = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>();

    app_state
        .db_context()
        .user_token()
        .register_token(&token, id)
        .await?;

    Ok(token)
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    phone_number: String,
    password: String,
}

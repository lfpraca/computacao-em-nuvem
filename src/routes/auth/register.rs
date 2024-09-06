use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{debug_handler, extract::State, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::{
    db::types::user::UserInsert,
    models::{app_state::AppState, user_role::UserRole},
};

use super::errors::register_error::RegisterError;

#[debug_handler]
pub async fn register_handler(
    State(app_state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<String, RegisterError> {
    if body.password.len() < 8 {
        return Err(RegisterError::InvalidPassword(
            "Senha tem menos de 8 dígitos".into(),
        ));
    }

    let salt = SaltString::generate(&mut thread_rng());
    let pass_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .inspect_err(|e| error!(error = ?&e, "Error hasing password"))?
        .to_string();

    let id = app_state
        .db_context()
        .user()
        .create_user(UserInsert {
            phone_number: &body.phone_number,
            name: &body.name,
            pass_hash: &pass_hash,
            address: &body.address,
            role: UserRole::User as i16,
        })
        .await?;

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

#[derive(Deserialize)]
pub struct RegisterRequest {
    phone_number: String,
    name: String,
    password: String,
    address: String,
}

use crate::errors::db_error::DbError;

use argon2::password_hash::Error as PasswordHashError;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    PasswordHashError(#[from] PasswordHashError),

    #[error("senha inválida: {0}")]
    InvalidPassword(String),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::PasswordHashError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno. Tente novamente mais tarde.",
            )
                .into_response(),

            Self::InvalidPassword(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response()
            }

            Self::DbError(e) => e.into_response(),
        }
    }
}

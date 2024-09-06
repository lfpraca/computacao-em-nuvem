use crate::errors::db_error::DbError;

use argon2::password_hash::Error as PasswordHashError;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    PasswordHashError(#[from] PasswordHashError),

    #[error("invalid password")]
    InvalidPassword,

    #[error("user not found")]
    NotFound,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::PasswordHashError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno. Tente novamente mais tarde.",
            )
                .into_response(),

            Self::InvalidPassword => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Senha incorreta.").into_response()
            }

            Self::NotFound => (StatusCode::NOT_FOUND, "Usuário não encontrado.").into_response(),

            Self::DbError(e) => e.into_response(),
        }
    }
}

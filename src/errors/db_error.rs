use axum::{http::StatusCode, response::IntoResponse};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    DieselError(#[from] DieselError),

    #[error(transparent)]
    Bb8RunError(#[from] RunError),

    #[error("unique violation in {constraint_name}")]
    ExpectedUniqueViolation { constraint_name: String },
}

impl IntoResponse for DbError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ExpectedUniqueViolation { .. } => {
                (StatusCode::CONFLICT, "Conflito com registros existentes.").into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno. Tente novamente mais tarde.",
            )
                .into_response(),
        }
    }
}

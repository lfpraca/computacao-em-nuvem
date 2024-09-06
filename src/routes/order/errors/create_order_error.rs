use crate::errors::db_error::DbError;

use axum::{http::StatusCode, response::IntoResponse};
use base64::DecodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateOrderError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    DecodeError(#[from] DecodeError),

    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

impl IntoResponse for CreateOrderError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DecodeError(_) => {
                (StatusCode::BAD_REQUEST, "Anexo em formato incorreto").into_response()
            }

            Self::HttpError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno. Tente novamente mais tarde.",
            )
                .into_response(),

            Self::DbError(e) => e.into_response(),
        }
    }
}

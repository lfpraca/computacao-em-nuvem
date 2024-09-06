use crate::errors::db_error::DbError;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetAttachmentError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    HttpError(#[from] reqwest::Error),

    #[error("{0}")]
    InternalError(String),
}

impl IntoResponse for GetAttachmentError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::HttpError(_) | Self::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno. Tente novamente mais tarde.",
            )
                .into_response(),

            Self::DbError(e) => e.into_response(),
        }
    }
}

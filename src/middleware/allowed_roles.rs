use std::future::Future;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};

use crate::models::token_data::TokenData;

const NO_PERMS: (StatusCode, &str) = (StatusCode::FORBIDDEN, "Usuário sem permissão");

pub async fn allowed_roles(
    Extension(user): Extension<TokenData>,
    req: Request<Body>,
    next: Next,
    allowed_roles: &[i16],
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    if !allowed_roles.contains(&user.role()) {
        return Err(NO_PERMS);
    }
    Ok(next.run(req).await)
}

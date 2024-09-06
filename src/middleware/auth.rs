use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::models::{app_state::AppState, token_data::TokenData};

const UNAUTH: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "Token inválido");

pub async fn auth(
    State(app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(UNAUTH)?
        .to_str()
        .map_err(|_| UNAUTH)?
        .strip_prefix("Token ")
        .ok_or(UNAUTH)?
        .to_string();

    let (user_id, role) = app_state
        .db_context()
        .user_token()
        .fetch_user_token_details(&token)
        .await
        .map_err(|_| UNAUTH)?;

    req.extensions_mut()
        .insert(TokenData::new(token, user_id, role));
    Ok(next.run(req).await)
}

use axum::{debug_handler, extract::State, Extension, Json};
use base64::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::types::order::OrderInsert,
    models::{app_state::AppState, token_data::TokenData},
    routes::order::errors::create_order_error::CreateOrderError,
    util::oci_sign::OciSign,
};

#[debug_handler]
pub async fn create_order_handler(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<TokenData>,
    Json(body): Json<CreateOrderRequest>,
) -> Result<String, CreateOrderError> {
    let id = Uuid::now_v7();
    let attachment_name = format!("{}.{}", id, body.attachment_extension);
    let request_url = format!(
        "https://objectstorage.sa-saopaulo-1.oraclecloud.com/n/{}/b/{}/o/{}",
        app_state.config().oci_namespace(),
        app_state.config().oci_bucket(),
        attachment_name,
    );

    let request = app_state
        .http_client()
        .put(request_url)
        .body(BASE64_STANDARD.decode(body.attachment_base64)?)
        .build()
        .unwrap()
        .sign_request(app_state.config());

    app_state.http_client().execute(request).await?;

    app_state
        .db_context()
        .order()
        .create_order(OrderInsert {
            id,
            amount: body.amount,
            user_id: token_data.user_id(),
            attachment_extension: &body.attachment_extension,
        })
        .await?;

    Ok(id.to_string())
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    amount: i16,
    attachment_extension: String,
    attachment_base64: String,
}

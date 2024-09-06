use std::ops::Add;

use axum::{
    debug_handler,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{json, Value};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    models::app_state::AppState,
    routes::admin::order::errors::get_attachment_error::GetAttachmentError,
    util::oci_sign::OciSign,
};

#[debug_handler]
pub async fn get_attachment_handler(
    State(app_state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<String, GetAttachmentError> {
    let attachment_extension = app_state
        .db_context()
        .order()
        .get_attachment_extension(order_id)
        .await?;

    let attachment_name = format!("{}.{}", order_id, attachment_extension);
    let request_url = format!(
        "https://objectstorage.sa-saopaulo-1.oraclecloud.com/n/{}/b/{}/p",
        app_state.config().oci_namespace(),
        app_state.config().oci_bucket(),
    );

    let request = app_state
        .http_client()
        .post(request_url)
        .json(&json!({
            "accessType": "ObjectRead",
            "name": format!("admin_get_attachment_{attachment_name}"),
            "objectName": attachment_name,
            "timeExpires": OffsetDateTime::now_utc().add(Duration::hours(1)).format(&Rfc3339).unwrap()
        }))
        .build()
        .unwrap()
        .sign_request(app_state.config());

    let response = app_state
        .http_client()
        .execute(request)
        .await
        .inspect_err(|e| error!(error = ?&e, "Erro sending http request"))?
        .json::<Value>()
        .await
        .inspect_err(|e| error!(error = ?&e, "Erro reading http json response body"))?;

    Ok(format!(
        "https://{}.objectstorage.sa-saopaulo-1.oci.customer-oci.com{}",
        app_state.config().oci_namespace(),
        response
            .get("accessUri")
            .ok_or_else(|| {
                GetAttachmentError::InternalError(
                    "accessUri was not present in reponse".to_string(),
                )
            })?
            .as_str()
            .unwrap()
    ))
}

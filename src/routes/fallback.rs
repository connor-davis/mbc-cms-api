use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

pub async fn get_fallback() -> Result<(StatusCode, impl IntoResponse), (StatusCode, Json<Value>)> {
    Ok((
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Route not found. Please contact the developer." })),
    ))
}

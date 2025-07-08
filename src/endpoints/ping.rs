use axum::{http::StatusCode, response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    status: String,
    message: String,
}

pub async fn ping_handler() -> Result<Json<PingResponse>, StatusCode> {
    let response = PingResponse {
        status: "ok".to_string(),
        message: "API is healthy and running!".to_string(),
    };

    Ok(Json(response))
}

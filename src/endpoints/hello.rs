use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct HelloQuery {
    name: Option<String>,
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello_handler(
    Query(params): Query<HelloQuery>,
) -> Result<Json<HelloResponse>, StatusCode> {
    let name = params.name.unwrap_or_else(|| "World".to_string());

    let response = HelloResponse {
        message: format!("Hello, {}! Welcome to our Axum Lambda API.", name),
    };

    Ok(Json(response))
}

pub async fn hello_post_handler(
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<HelloResponse>, StatusCode> {
    let name = payload
        .get("name")
        .cloned()
        .unwrap_or_else(|| "World".to_string());

    let response = HelloResponse {
        message: format!("Hello via POST, {}! Your message was received.", name),
    };

    Ok(Json(response))
}

use axum::{
    Router,
    response::Json,
    routing::{get, post},
};
use lambda_http::run;
use serde_json::{Value, json};
use tower_http::cors::CorsLayer;

mod endpoints;

use endpoints::{
    hello::{hello_handler, hello_post_handler},
    users::{create_user, get_user_by_id, get_users},
};

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    // Build our application with routes
    let app = Router::new()
        // Hello endpoints
        .route("/", get(root))
        .route("/hello", get(hello_handler))
        .route("/hello", post(hello_post_handler))
        // Users endpoints
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        // Health check endpoint
        // Add CORS support
        .layer(CorsLayer::permissive());

    run(app).await
}

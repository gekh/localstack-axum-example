use axum::{
    Router,
    routing::{get, post},
};
use lambda_http::run;
use tower_http::cors::CorsLayer;

mod endpoints;

use endpoints::{
    hello::{hello_handler, hello_post_handler},
    users::{create_user, get_user_by_id, get_users},
};

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    // Build our application with routes
    let app = Router::new()
        // Hello endpoints
        .route("/hello", get(hello_handler))
        .route("/hello", post(hello_post_handler))
        // Users endpoints
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        // Health check endpoint
        .route("/", get(|| async { "OK" }))
        // Add CORS support
        .layer(CorsLayer::permissive());

    run(app).await
}

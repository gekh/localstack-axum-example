use axum::{
    Router,
    routing::{get, post},
};
use lambda_http::{Body, Error, Request, run, service_fn};
use tower::ServiceExt;

mod endpoints;

use endpoints::{
    hello::{hello_handler, hello_post_handler},
    ping::ping_handler,
    users::{create_user, get_user_by_id, get_users},
};

fn app() -> Router {
    Router::new()
        // Hello endpoints
        .route("/", get(ping_handler))
        .route("/hello", get(hello_handler))
        .route("/hello", post(hello_post_handler))
        // Users endpoints
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        // Health check endpoint
        .route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "OK - Axum Lambda API is running!"
}

async fn function_handler(event: Request) -> Result<lambda_http::Response<Body>, Error> {
    let app = app();

    // Convert lambda_http::Request to axum-compatible http::Request
    let (parts, body) = event.into_parts();
    let body = axum::body::Body::new(body);
    let request = axum::http::Request::from_parts(parts, body);

    // Call the axum app
    let response = app
        .oneshot(request)
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    // Convert axum Response to lambda_http Response
    let (parts, body) = response.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    let lambda_response = lambda_http::Response::from_parts(parts, body_bytes.to_vec().into());

    Ok(lambda_response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

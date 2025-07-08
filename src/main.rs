use lambda_http::{Body, Error, Request, Response, run, service_fn};

async fn function_handler(_request: Request) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello from Lambda!".into())
        .map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

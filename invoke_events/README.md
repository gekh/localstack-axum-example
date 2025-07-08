# Mock Invoke Events

This folder contains mock AWS API Gateway events that can be used to test your Lambda function locally or for integration testing.

## Files

- `health_check.json` - GET / (health check)
- `hello_get.json` - GET /hello?name=John
- `hello_post.json` - POST /hello with JSON body
- `users_get.json` - GET /users?limit=5&offset=0
- `users_post.json` - POST /users with JSON body to create a user
- `users_get_by_id.json` - GET /users/1 to get a specific user

## Usage

### With AWS SAM CLI

```bash
# Test the health check endpoint
sam local invoke --event invoke_events/health_check.json

# Test the hello GET endpoint
sam local invoke --event invoke_events/hello_get.json

# Test the hello POST endpoint
sam local invoke --event invoke_events/hello_post.json

# Test the users GET endpoint
sam local invoke --event invoke_events/users_get.json

# Test the users POST endpoint
sam local invoke --event invoke_events/users_post.json

# Test the users get by ID endpoint
sam local invoke --event invoke_events/users_get_by_id.json
```

### With lambda_runtime testing

You can also use these events in your Rust unit tests by loading the JSON files and passing them to your handler function for integration testing.

```rust
#[tokio::test]
async fn test_health_check_endpoint() {
    let event_json = std::fs::read_to_string("invoke_events/health_check.json").unwrap();
    let event: lambda_http::Request = serde_json::from_str(&event_json).unwrap();

    let response = function_handler(event).await.unwrap();
    // Assert response...
}
```

## Event Structure

These events follow the AWS API Gateway Lambda proxy integration format version 2.0, which includes:

- HTTP method and path
- Headers
- Query string parameters (where applicable)
- Path parameters (where applicable)
- Request body (for POST requests)
- Request context with metadata

## Customizing Events

You can modify these files to test different scenarios:

- Change query parameters
- Modify request bodies
- Test different path parameters
- Add or remove headers
- Test error conditions

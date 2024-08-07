use axum::extract::Query;
use serde::Deserialize;

// Define a struct to hold the query parameters for the healthcheck endpoint
#[derive(Debug, Deserialize)]
pub struct Healthcheck {
    // An optional field that may contain the 'name' parameter from the query
    name: Option<String>,
}

// Define an asynchronous function that acts as the handler for the healthcheck
// endpoint
pub async fn healthcheck(options: Option<Query<Healthcheck>>) -> String {
    // Format the response message
    // If 'options' is None or 'name' is None, use "world" as the default value
    format!(
        "Hello, {}!",
        // Use unwrap_or to provide a default value if 'name' is None
        options.unwrap().name.clone().unwrap_or("world".to_string())
    )
}

// Example Requests and Responses

// 1. With name parameter:
// Request: GET /healthcheck?name=Alice
// Response: Hello, Alice!

// 2. Without name parameter:
// Request: GET /healthcheck
// Response: Hello, world!

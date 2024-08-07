use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{BoxError, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// Import custom utility module (assumed to contain the healthcheck function)
mod utils;

#[tokio::main]
async fn main() {
    // Set up a tracing subscriber with a maximum log level of DEBUG
    let subscriber = FmtSubscriber::builder().with_max_level(Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default subscriber");

    // Define the application router with a single route and layered middleware
    let app = Router::new().route("/", get(utils::healthcheck::healthcheck)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| {
                async move {
                    // Handle timeout errors separately
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        // Handle other internal server errors
                        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}")))
                    }
                }
            }))
            // Set a timeout for requests
            .timeout(Duration::from_secs(10))
            // Add tracing layer for logging request details
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );

    // Bind the application to a TCP listener on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Log the listening address
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}

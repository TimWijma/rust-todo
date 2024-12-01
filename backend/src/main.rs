use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use http::header::HeaderValue;

#[tokio::main]
async fn main() {
    // Build the router
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Define the server address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, Axum!"
}

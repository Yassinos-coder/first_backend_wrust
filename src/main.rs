
use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse},
    extract::Json,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Define the router with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/signup", post(signup))
        .route("/isAlive", get(is_alive));

    // Bind the listener
    let addr = "127.0.0.1:8009".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);

    // Start the Axum server
    axum::serve(listener, app).await.unwrap();
}

// Define a struct to extract JSON data from the request body
#[derive(Deserialize)]
struct SignupData {
    username: String,
    password: String,
}

async fn is_alive() -> Json<serde_json::Value> {
    Json(json!({
        "database": "on",
        "server": "on"
    }))
}

// Corrected signup function
async fn signup(Json(data): Json<SignupData>) -> &'static str {
    println!("Received signup request: username={}, password={}", data.username, data.password);
    "Signup successful"
}

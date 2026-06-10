use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Welcome, cyber traveler!" }));
        
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("tokio::net::TcpListener could not bind to port 3000");
    axum::serve(listener, app).await.expect("axum::serve could not create service");
}

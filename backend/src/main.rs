use axum::{routing::{get, post}, Router};
use sqlx::sqlite::SqlitePoolOptions;

mod model;
mod handlers;

use model::*;

#[tokio::main]
async fn main() {

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:anisquiz.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/", get(|| async { "Welcome, cyber traveler!" }))
        .route("/games", post(Game::create).get(Game::list))
        .route("/g/{game_id}", get(Game::get))
        .route("/g/{game_id}/t/{team_id}", get(|| async { "This is team"}))
        .route("/g/{game_id}/q/{id}", get(|| async { "This is quest"}))
        .with_state(pool);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("tokio::net::TcpListener could not bind to port 3000");
    axum::serve(listener, app).await.expect("axum::serve could not create service");
}

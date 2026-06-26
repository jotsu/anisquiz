use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use axum::{
    Router,
    routing::{get, post},
    middleware::from_fn_with_state,
};

mod model;
mod api;
mod errors;

use model::{User, Game, Team, Quest, LogEntry};
use api::auth::auth_middleware;
pub use errors::AppError;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
    // api_key: String,
    jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let options = SqliteConnectOptions::new()
        .filename(&database_url)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await
        .expect(format!("Failed to connect to database {}", &database_url).as_str());
        sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let state = AppState { pool, jwt_secret };

    let protected = Router::new()
        .route("/games", post(Game::create).get(Game::list))
        .route("/g/{game_id}", get(Game::get))
        .route("/g/{game_id}/teams", post(Team::create).get(Team::list))
        .route("/g/{game_id}/t/{team_id}", get(Team::get))
        .route("/g/{game_id}/quests", post(Quest::create).get(Quest::list))
        .route("/g/{game_id}/q/{quest_id}", get(Quest::get))
        .route("/g/{game_id}/logs", post(LogEntry::create).get(LogEntry::list))
        .route("/g/{game_id}/l/{log_id}", get(LogEntry::get))
        .route_layer(from_fn_with_state(state.clone(), auth_middleware));
    
    let public = Router::new()
        .route("/register", post(User::register))
        .route("/login", post(User::login));

    let app = Router::new()
        .merge(protected)
        .merge(public)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("tokio::net::TcpListener could not bind to port 3000");
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("axum::serve could not create service");
}
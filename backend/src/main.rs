use sqlx::sqlite::SqlitePool;
use axum::{
    Router,
    routing::{get, post},
    middleware::{Next, from_fn_with_state},
    extract::State,
    http::Request,
    response::Response,
};

mod model;
mod errors;

use model::{game::Game, team::Team, quest::Quest, log::LogEntry};
use errors::AppError;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let api_key = std::env::var("API_KEY")
        .expect("API_KEY must be set");
    let pool = SqlitePool::connect(&database_url).await
        .expect(format!("Failed to connect to database {}", &database_url).as_str());
    
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState { pool, api_key };

    let protected = Router::new()
        .route("/games", post(Game::create).get(Game::list))
        .route("/g/{game_id}", get(Game::get))
        .route("/g/{game_id}/teams", post(Team::create).get(Team::list))
        .route("/g/{game_id}/t/{team_id}", get(Team::get))
        .route("/g/{game_id}/quests", post(Quest::create).get(Quest::list))
        .route("/g/{game_id}/q/{quest_id}", get(Quest::get))
        .route("/g/{game_id}/logs", post(LogEntry::create).get(LogEntry::list))
        .route("/g/{game_id}/log/{log_id}", get(LogEntry::get))
        .route_layer(from_fn_with_state(state.clone(),require_api_key));
    
    let public = Router::new()
        .route("/", get(|| async { "Welcome, cyber traveler!" }));

    let app = Router::new()
        .merge(protected)
        .merge(public)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("tokio::net::TcpListener could not bind to port 3000");
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("axum::serve could not create service");
}


#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
    api_key: String,
}

pub async fn require_api_key(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            if token == state.api_key {
                Ok(next.run(request).await)
            } else {
                Err(AppError::unauthorized("Invalid API key"))
            }
        }
        _ => Err(AppError::unauthorized("API key not provided")),
    }
}
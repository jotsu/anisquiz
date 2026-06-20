use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use crate::{
    AppState,
    errors::AppError,
};

// ----- DATABASE OBJECT -----
#[derive(Serialize, FromRow)]
pub struct Game {
    id: String,
    title: String,
    created_at: NaiveDateTime,
    round: u32,
    turn: u32,
    active_team_id: Option<String>,
    active_quest_id: Option<String>,
}

// ----- REQUEST PAYLOADS -----
#[derive(Deserialize)]
pub struct CreateGame {
    title: String,
}

// ----- REQUEST HANDLERS -----
impl Game {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateGame>,
    ) -> Result<(StatusCode, Json<Game>), AppError> {
        let id = nanoid::nanoid!(6);
        let row = sqlx::query_as::<_, Game>(
            "INSERT INTO games (id, title) VALUES (?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&payload.title)
        .fetch_one(&state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(state): State<AppState>,
    ) -> Result<Json<Vec<Game>>, AppError> {
    let rows = sqlx::query_as::<_, Game>(
        "SELECT * FROM games ORDER BY created_at DESC"
        )
        .fetch_all(&state.pool)
        .await?;
        Ok(Json(rows))
    }

    pub async fn get(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Game>, AppError> {
        let row = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ?"
        )
        .bind(&game_id)
        .fetch_optional(&state.pool)
        .await?;
        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Game with id '{}' not found", game_id)))
    }

}
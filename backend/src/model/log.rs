use crate::{AppState, AppError};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};

// ----- DATABASE OBJECT -----
#[derive(Serialize, FromRow)]
pub struct LogEntry {
    id: String,
    parent_game_id: String,
    created_at: NaiveDateTime,
    message: String,
}

// ----- REQUEST PAYLOADS -----
#[derive(Deserialize)]
pub struct CreateLogEntry {
    message: String,
}

// ----- REQUEST HANDLERS -----
impl LogEntry {
    pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateLogEntry>,
    ) -> Result<(StatusCode, Json<LogEntry>), AppError> {
        let id = nanoid::nanoid!(15);
        let row = sqlx::query_as::<_, LogEntry>(
            "INSERT INTO logs (id, parent_game_id, message) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.message)
        .fetch_one(&state.pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<LogEntry>>, AppError> {
    let rows = sqlx::query_as::<_, LogEntry>(
        "SELECT * FROM logs WHERE parent_game_id = ? ORDER BY created_at DESC"
        )
        .bind(&game_id)
        .fetch_all(&state.pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, log_id)): Path<(String, String)>,
    ) -> Result<Json<LogEntry>, AppError> {
        let row = sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&log_id)
        .fetch_optional(&state.pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Log entry with id '{}' for game with id '{}' not found", log_id, game_id)))
    }
}
use crate::{AppState, AppError};
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};

// ----- DATABASE OBJECT -----
#[derive(Serialize, FromRow)]
pub struct Quest {
    id: String,
    parent_game_id: String,
    no: u32,
    src: String,
    pts: i32,
}

// ----- REQUEST PAYLOADS -----
#[derive(Deserialize)]
pub struct CreateQuest {
    no: u32,
    src: String,
}

// ----- REQUEST HANDLERS -----
impl Quest {
        pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateQuest>,
    ) -> Result<(StatusCode, Json<Quest>), AppError> {
        let id = nanoid::nanoid!(12);
        let row = sqlx::query_as::<_, Quest>(
            "INSERT INTO quests (id, parent_game_id, no, src) VALUES (?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.no)
        .bind(&payload.src)
        .fetch_one(&state.pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Quest>>, AppError> {
    let rows = sqlx::query_as::<_, Quest>(
        "SELECT * FROM quests WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(&game_id)
        .fetch_all(&state.pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, quest_id)): Path<(String, String)>,
    ) -> Result<Json<Quest>, AppError> {
        let row = sqlx::query_as::<_, Quest>(
            "SELECT * FROM quests WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&quest_id)
        .fetch_optional(&state.pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Quest with id '{}' for game with id '{}' not found", quest_id, game_id)))
    }
}
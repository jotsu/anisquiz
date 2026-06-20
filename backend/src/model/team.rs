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
pub struct Team {
    id: String,
    parent_game_id: String,
    no: u32,
    name: String,
    score: i32,
}

// ----- REQUEST PAYLOADS -----
#[derive(Deserialize)]
pub struct CreateTeam {
    no: u32,
    name: String,
}

// ----- REQUEST HANDLERS -----
impl Team {
        pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateTeam>,
    ) -> Result<(StatusCode, Json<Team>), AppError> {
        let id = nanoid::nanoid!(9);
        let row = sqlx::query_as::<_, Team>(
            "INSERT INTO teams (id, parent_game_id, no, name) VALUES (?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.no)
        .bind(&payload.name)
        .fetch_one(&state.pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Team>>, AppError> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(game_id)
        .fetch_all(&state.pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, team_id)): Path<(String, String)>,
    ) -> Result<Json<Team>, AppError> {
        let row = sqlx::query_as::<_, Team>(
            "SELECT * FROM teams WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&team_id)
        .fetch_optional(&state.pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Team with id '{}' for game with id '{}' not found", team_id, game_id)))
    }
}
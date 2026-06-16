use sqlx::SqlitePool;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Deserialize;
use super::model::*;
use super::errors::AppError;

//REQUEST PAYLOADS
#[derive(Deserialize)]
pub struct CreateGame {
    title: String,
}

#[derive(Deserialize)]
pub struct CreateTeam {
    no: u32,
    name: String,
}

#[derive(Deserialize)]
pub struct CreateQuest {
    no: u32,
    src: String,
}

#[derive(Deserialize)]
pub struct CreateLogEntry {
    message: String,
}

//HANDLERS
impl Game {
    pub async fn create(
        State(pool): State<SqlitePool>,
        Json(payload): Json<CreateGame>,
    ) -> Result<(StatusCode, Json<Game>), AppError> {
        let id = nanoid::nanoid!(6);
        let row = sqlx::query_as::<_, Game>(
            "INSERT INTO games (id, title) VALUES (?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&payload.title)
        .fetch_one(&pool)
        .await?;

        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
    ) -> Result<Json<Vec<Game>>, AppError> {
    let rows = sqlx::query_as::<_, Game>(
        "SELECT * FROM games ORDER BY created_at DESC"
        )
        .fetch_all(&pool)
        .await?;
        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Game>, AppError> {
        let row = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ?"
        )
        .bind(&game_id)
        .fetch_optional(&pool)
        .await?;
        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Game with id '{}' not found", game_id)))
    }

}

impl Team {
        pub async fn create(
        State(pool): State<SqlitePool>,
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
        .fetch_one(&pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Team>>, AppError> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(game_id)
        .fetch_all(&pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, team_id)): Path<(String, String)>,
    ) -> Result<Json<Team>, AppError> {
        let row = sqlx::query_as::<_, Team>(
            "SELECT * FROM teams WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&team_id)
        .fetch_optional(&pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Team with id '{}' for game with id '{}' not found", team_id, game_id)))
    }
}

impl Quest {
        pub async fn create(
        State(pool): State<SqlitePool>,
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
        .fetch_one(&pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Quest>>, AppError> {
    let rows = sqlx::query_as::<_, Quest>(
        "SELECT * FROM quests WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(&game_id)
        .fetch_all(&pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, quest_id)): Path<(String, String)>,
    ) -> Result<Json<Quest>, AppError> {
        let row = sqlx::query_as::<_, Quest>(
            "SELECT * FROM quests WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&quest_id)
        .fetch_optional(&pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Quest with id '{}' for game with id '{}' not found", quest_id, game_id)))
    }
}

impl LogEntry {
    pub async fn create(
        State(pool): State<SqlitePool>,
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
        .fetch_one(&pool)
        .await?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Team>>, AppError> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM logs WHERE parent_game_id = ? ORDER BY created_at DESC"
        )
        .bind(&game_id)
        .fetch_all(&pool)
        .await?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, log_id)): Path<(String, String)>,
    ) -> Result<Json<LogEntry>, AppError> {
        let row = sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&log_id)
        .fetch_optional(&pool)
        .await?;

        row.map(|r| Json(r))
            .ok_or_else(|| AppError::not_found(format!("Log entry with id '{}' for game with id '{}' not found", log_id, game_id)))
    }
}
use sqlx::SqlitePool;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Deserialize;
use super::model::*;

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
    ) -> Result<(StatusCode, Json<Game>), StatusCode> {
        let id = nanoid::nanoid!(6);
        let row = sqlx::query_as::<_, Game>(
            "INSERT INTO games (id, title) VALUES (?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&payload.title)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
    ) -> Result<Json<Vec<Game>>, StatusCode> {
    let rows = sqlx::query_as::<_, Game>(
        "SELECT * FROM games ORDER BY created_at DESC"
        )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Game>, StatusCode> {
        let row = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ?"
        )
        .bind(&game_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        row.map(|r| Json(r))
            .ok_or(StatusCode::NOT_FOUND)
    }

}

impl Team {
        pub async fn create(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateTeam>,
    ) -> Result<(StatusCode, Json<Team>), StatusCode> {
        let id = nanoid::nanoid!(9);
        let row = sqlx::query_as::<_, Team>(
            "INSERT INTO teams (id, parent_game_id, no, name) VALUES (?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.no)
        .bind(&payload.name)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Team>>, StatusCode> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(game_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, team_id)): Path<(String, String)>,
    ) -> Result<Json<Team>, StatusCode> {
        let row = sqlx::query_as::<_, Team>(
            "SELECT * FROM teams WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&team_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        row.map(|r| Json(r))
            .ok_or(StatusCode::NOT_FOUND)
    }
}

impl Quest {
        pub async fn create(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateQuest>,
    ) -> Result<(StatusCode, Json<Quest>), StatusCode> {
        let id = nanoid::nanoid!(12);
        let row = sqlx::query_as::<_, Quest>(
            "INSERT INTO quests (id, parent_game_id, no, src) VALUES (?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.no)
        .bind(&payload.src)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Quest>>, StatusCode> {
    let rows = sqlx::query_as::<_, Quest>(
        "SELECT * FROM quests WHERE parent_game_id = ? ORDER BY no ASC"
        )
        .bind(&game_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, quest_id)): Path<(String, String)>,
    ) -> Result<Json<Quest>, StatusCode> {
        let row = sqlx::query_as::<_, Quest>(
            "SELECT * FROM quests WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&quest_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        row.map(|r| Json(r))
            .ok_or(StatusCode::NOT_FOUND)
    }
}

impl LogEntry {
    pub async fn create(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateLogEntry>,
    ) -> Result<(StatusCode, Json<LogEntry>), StatusCode> {
        let id = nanoid::nanoid!(16);
        let row = sqlx::query_as::<_, LogEntry>(
            "INSERT INTO logs (id, parent_game_id, message) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&game_id)
        .bind(&payload.message)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Vec<Team>>, StatusCode> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM logs WHERE parent_game_id = ? ORDER BY created_at DESC"
        )
        .bind(&game_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(rows))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path((game_id, log_id)): Path<(String, String)>,
    ) -> Result<Json<LogEntry>, StatusCode> {
        let row = sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&log_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        row.map(|r| Json(r))
            .ok_or(StatusCode::NOT_FOUND)
    }
}
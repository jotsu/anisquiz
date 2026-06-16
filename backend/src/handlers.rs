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
        "SELECT * FROM teams WHERE parent_game_id = ? ORDER BY no DESC"
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
    ) -> Result<(StatusCode, Json<Team>), StatusCode> {
        let id = nanoid::nanoid!(9);
        let row = sqlx::query_as::<_, Team>(
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
    ) -> Result<Json<Vec<Team>>, StatusCode> {
    let rows = sqlx::query_as::<_, Team>(
        "SELECT * FROM quests WHERE parent_game_id = ? ORDER BY no DESC"
        )
        .bind(&game_id)
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
            "SELECT * FROM quests WHERE parent_game_id = ? AND id = ?"
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
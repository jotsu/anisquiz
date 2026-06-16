use sqlx::SqlitePool;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Deserialize;
use super::model::*;



//POST payloads
#[derive(Deserialize)]
pub struct CreateGame {
    title: String,
}

//HANDLERS
impl Game {
    pub async fn create(
        State(pool): State<SqlitePool>,
        Json(payload): Json<CreateGame>,
    ) -> Result<(StatusCode, Json<Game>), StatusCode> {
        let id = nanoid::nanoid!(6);

        let game_row = sqlx::query_as::<_, Game>(
            "INSERT INTO games (id, title) VALUES (?, ?) RETURNING id, title, created_at, round, turn, active_team_id, active_quest_id"
        )
        .bind(&id)
        .bind(&payload.title)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok((StatusCode::CREATED, Json(game_row)))
    }

    pub async fn list(
        State(pool): State<SqlitePool>,
    ) -> Result<Json<Vec<Game>>, StatusCode> {
    let games = sqlx::query_as::<_, Game>(
        "SELECT id, title, created_at, round, turn, active_team_id, active_quest_id FROM games ORDER BY created_at DESC"
        )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(games))
    }

    pub async fn get(
        State(pool): State<SqlitePool>,
        Path(game_id): Path<String>,
    ) -> Result<Json<Game>, StatusCode> {
        let game_row = sqlx::query_as::<_, Game>(
            "SELECT id, title, created_at, round, turn, active_team_id, active_quest_id FROM games WHERE id = ?"
        )
        .bind(&game_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        game_row.map(|r| Json(r))
            .ok_or(StatusCode::NOT_FOUND)
    }

}
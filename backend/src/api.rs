use serde::Deserialize;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;
use crate::{AppState, AppError, model::*};

pub mod auth;
pub mod types;

use auth::*;
use types::*;

impl User {
    pub async fn register(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUser>,
    ) -> Result<(StatusCode, Json<User>), AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password(&payload.password);
        let row = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&payload.name)
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await?;
        sqlx::query_as::<_, Password>(
            "INSERT INTO passwords (user_id, password_hash) VALUES (?, ?)"
        )
        .bind(&id)
        .bind(&hash)
        .fetch_one(&state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginUser>
    ) -> Result<Json<LoginResponse>, AppError> {
        let user = sqlx::query_as::<_, User>(
        "SELECT id FROM users WHERE email = ?"
        )
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::not_found(format!("User with email '{}' not found.", &payload.email)))?;

        let stored_password = sqlx::query_as::<_, Password>(
        "SELECT password_hash FROM passwords WHERE user_id = ?"
        )
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::unauthorized("Failed to authenticate: Password for user with email '{}' not set."))?;

        if verify_password(&payload.password, &stored_password.password_hash) {
            let token = create_token(&state, &user.id, &user.role)?;
            let expires_in = 24 * 3600;
            Ok(Json(LoginResponse{token, expires_in}))
        } else {
            Err(AppError::unauthorized("Failed to authenticate: Password for user with email '{}' not set."))
        }
    }
}


impl Game {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateGame>,
    ) -> Result<(StatusCode, Json<Game>), AppError> {
        let id = Uuid::new_v4().to_string();
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
        State(state): State<AppState>
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


impl Team {
        pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateTeam>,
    ) -> Result<(StatusCode, Json<Team>), AppError> {
        let id = Uuid::new_v4().to_string();
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
            .ok_or(AppError::not_found(format!("Team with id '{}' for game with id '{}' not found", team_id, game_id)))
    }
}


impl Quest {
        pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateQuest>,
    ) -> Result<(StatusCode, Json<Quest>), AppError> {
        let id = Uuid::new_v4().to_string();
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
            .ok_or(AppError::not_found(format!("Quest with id '{}' for game with id '{}' not found", quest_id, game_id)))
    }
}



#[derive(Deserialize)]
pub struct CreateLogEntry {
    message: String,
}

impl LogEntry {
    pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Json(payload): Json<CreateLogEntry>,
    ) -> Result<(StatusCode, Json<LogEntry>), AppError> {
        let id = Uuid::new_v4().to_string();
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
        sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? AND id = ?"
        )
        .bind(&game_id)
        .bind(&log_id)
        .fetch_optional(&state.pool)
        .await?
        .map(|r| Json(r))
        .ok_or(AppError::not_found(format!("Log entry with id '{}' for game with id '{}' not found", log_id, game_id)))
    }
}
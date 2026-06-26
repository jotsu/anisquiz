use axum::{Extension, Json, extract::{Path, State}, http::StatusCode};
use chrono::prelude::*;
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
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&uuid::Uuid::new_v4().to_string())
        .bind(&payload.name)
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await?;

        sqlx::query(
            "INSERT INTO passwords (user_id, password_hash) VALUES (?, ?)"
        )
        .bind(&user.id)
        .bind(&hash_password(&payload.password))
        .execute(&state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(user)))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginUser>
    ) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
        let user = sqlx::query_as::<_, User>(
        "SELECT id FROM users WHERE email = ?"
        )
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::unauthorized("Failed to authenticate: User not found."))?;
        
        if !user.verified {
            return Err(AppError::unauthorized("Failed to authenticate: User not verified."));
        }
        
        let stored_password = sqlx::query_as::<_, Password>(
        "SELECT password_hash FROM passwords WHERE user_id = ?"
        )
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::unauthorized("Failed to authenticate: Password not set."))?;
        
        if !verify_password(&payload.password, &stored_password.password_hash) {
            return Err(AppError::unauthorized("Failed to authenticate: Wrong username or password."));
        }

        let response = LoginResponse {
            token: create_token(&state, &user.id, &user.role)?,
            expires_in: 24 * 3600,
        };

        Ok((StatusCode::OK, Json(response)))
    }
}


impl Game {
    pub async fn create(
        State(state): State<AppState>,
        Extension(user): Extension<CurrentUser>,
        Json(payload): Json<CreateGame>,
    ) -> Result<(StatusCode, Json<Game>), AppError> {
        let id = Uuid::new_v4().to_string();
        let row = sqlx::query_as::<_, Game>(
            "INSERT INTO games (id, owner_id, title) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&payload.title)
        .bind(&user.id)
        .fetch_one(&state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(row)))
    }

    pub async fn list(
        State(state): State<AppState>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<(StatusCode, Json<Vec<Game>>), AppError> {
        let games = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE owner_id = ? ORDER BY created_at DESC"
        )
        .bind(&user.id)
        .fetch_all(&state.pool)
        .await?;

        Ok((StatusCode::OK, Json(games)))
    }

    pub async fn get(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<(StatusCode, Json<Game>), AppError> {
        if let Some(game) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            Ok((StatusCode::OK, Json(game)))
        } else {
            Err(AppError::not_found("Game not found."))
        }
    }
}


impl Team {
        pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
        Json(payload): Json<CreateTeam>,
    ) -> Result<(StatusCode, Json<Team>), AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let id = Uuid::new_v4().to_string();
            let team = sqlx::query_as::<_, Team>(
                "INSERT INTO teams (id, parent_game_id, no, name) VALUES (?, ?, ?, ?) RETURNING *"
            )
            .bind(&id)
            .bind(&game_id)
            .bind(&payload.no)
            .bind(&payload.name)
            .fetch_one(&state.pool)
            .await?;

            Ok((StatusCode::CREATED, Json(team)))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<Json<Vec<Team>>, AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let teams = sqlx::query_as::<_, Team>(
            "SELECT * FROM teams WHERE parent_game_id = ? ORDER BY no ASC"
            )
            .bind(&game_id)
            .fetch_all(&state.pool)
            .await?;

            Ok(Json(teams))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, team_id)): Path<(String, String)>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<(StatusCode, Json<Team>), AppError> {
        if let (Some(_), Some(team)) = (
            sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
            )
            .bind(&game_id)
            .bind(&user.id)
            .fetch_optional(&state.pool)
            .await?,
            sqlx::query_as::<_, Team>(
            "SELECT * FROM teams WHERE parent_game_id = ? AND id = ?"
            )
            .bind(&game_id)
            .bind(&team_id)
            .fetch_optional(&state.pool)
            .await?
        ) {
            Ok((StatusCode::OK, Json(team)))
        } else {
            Err(AppError::not_found("Team not found."))
        }
    }
}


impl Quest {
    pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
        Json(payload): Json<CreateQuest>,
    ) -> Result<(StatusCode, Json<Quest>), AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let id = Uuid::new_v4().to_string();
            let quest = sqlx::query_as::<_, Quest>(
                "INSERT INTO quests (id, parent_game_id, no, src) VALUES (?, ?, ?, ?) RETURNING *"
            )
            .bind(&id)
            .bind(&game_id)
            .bind(&payload.no)
            .bind(&payload.src)
            .fetch_one(&state.pool)
            .await?;

            Ok((StatusCode::CREATED, Json(quest)))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<Json<Vec<Quest>>, AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let quests = sqlx::query_as::<_, Quest>(
            "SELECT * FROM quests WHERE parent_game_id = ? ORDER BY no ASC"
            )
            .bind(&game_id)
            .fetch_all(&state.pool)
            .await?;

            Ok(Json(quests))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, quest_id)): Path<(String, String)>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<(StatusCode, Json<Quest>), AppError> {
        if let (Some(_), Some(quest)) = (
            sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
            )
            .bind(&game_id)
            .bind(&user.id)
            .fetch_optional(&state.pool)
            .await?,
            sqlx::query_as::<_, Quest>(
            "SELECT * FROM quests WHERE parent_game_id = ? AND id = ?"
            )
            .bind(&game_id)
            .bind(&quest_id)
            .fetch_optional(&state.pool)
            .await?
        ) {
            Ok((StatusCode::OK, Json(quest)))
        } else {
            Err(AppError::not_found("Quest not found."))
        }
    }
}


impl LogEntry {
    pub async fn create(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
        Json(payload): Json<CreateLogEntry>,
    ) -> Result<(StatusCode, Json<LogEntry>), AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let id = Uuid::new_v4().to_string();
            let created_at = Utc::now();
            let log = sqlx::query_as::<_, LogEntry>(
                "INSERT INTO logs (id, parent_game_id, created_at, message) VALUES (?, ?, ?, ?) RETURNING *"
            )
            .bind(&id)
            .bind(&game_id)
            .bind(&created_at)
            .bind(&payload.message)
            .fetch_one(&state.pool)
            .await?;

            Ok((StatusCode::CREATED, Json(log)))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn list(
        State(state): State<AppState>,
        Path(game_id): Path<String>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<Json<Vec<LogEntry>>, AppError> {
        if let Some(_) = sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
        )
        .bind(&game_id)
        .bind(&user.id)
        .fetch_optional(&state.pool)
        .await? {
            let logs = sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? ORDER BY no ASC"
            )
            .bind(&game_id)
            .fetch_all(&state.pool)
            .await?;

            Ok(Json(logs))
        } else {
            Err(AppError::not_found("Parent game not found."))
        }
    }

    pub async fn get(
        State(state): State<AppState>,
        Path((game_id, log_id)): Path<(String, String)>,
        Extension(user): Extension<CurrentUser>,
    ) -> Result<(StatusCode, Json<LogEntry>), AppError> {
        if let (Some(_), Some(log)) = (
            sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE id = ? AND owner_id = ?"
            )
            .bind(&game_id)
            .bind(&user.id)
            .fetch_optional(&state.pool)
            .await?,
            sqlx::query_as::<_, LogEntry>(
            "SELECT * FROM logs WHERE parent_game_id = ? AND id = ?"
            )
            .bind(&game_id)
            .bind(&log_id)
            .fetch_optional(&state.pool)
            .await?
        ) {
            Ok((StatusCode::OK, Json(log)))
        } else {
            Err(AppError::not_found("LogEntry not found."))
        }
    }
}
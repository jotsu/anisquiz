use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub role: String,
    pub photo: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Password {
    pub user_id: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Game {
    pub id: String,
    pub owner_user_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub round: u32,
    pub turn: u32,
    pub active_team_id: Option<String>,
    pub active_quest_id: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Team {
    pub id: String,
    pub parent_game_id: String,
    pub no: u32,
    pub name: String,
    pub score: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Quest {
    pub id: String,
    pub parent_game_id: String,
    pub no: u32,
    pub src: String,
    pub pts: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct LogEntry {
    pub id: String,
    pub parent_game_id: String,
    pub created_at: NaiveDateTime,
    pub message: String,
}
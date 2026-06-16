use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, FromRow)]
pub struct Game {
    id: String,
    title: String,
    created_at: NaiveDateTime,
    round: u32,
    turn: u32,
    active_team_id: Option<String>,
    active_quest_id: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct Team {
    id: String,
    parent_game_id: String,
    no: usize,
    name: String,
    score: i32,
}

#[derive(Serialize, FromRow)]
pub struct Quest {
    id: String,
    parent_game_id: String,
    no: usize,
    src: String,
    pts: i32,
}

#[derive(Serialize, FromRow)]
struct LogEntry {
    id: String,
    parent_game_id: String,
    created_at: NaiveDateTime,
    message: String,
}

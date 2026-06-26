use chrono::prelude::*;
use yew::{Properties, UseStateHandle};
use serde::{Serialize, Deserialize};

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub state: UseStateHandle<GameState>,
    pub token: String,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub game: Game,
    pub teams: Vec::<Team>,
    pub quests: Vec::<Quest>,
    pub logs: Vec::<LogEntry>,
}

#[derive(Clone, PartialEq)]
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

#[derive(Deserialize, Clone, PartialEq)]
pub struct Game {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub round: u32,
    pub turn: u32,
    pub active_team_id: Option<String>,
    pub active_quest_id: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Team {
    pub id: String,
    pub parent_game_id: String,
    pub no: u32,
    pub name: String,
    pub score: i32,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Quest {
    pub id: String,
    pub parent_game_id: String,
    pub no: u32,
    pub src: String,
    pub pts: i32,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LogEntry {
    pub id: String,
    pub parent_game_id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}
use serde::{Serialize, Deserialize};
use yew::Properties;
use chrono::NaiveDateTime;

// ----- OBJECTS -----

#[derive(Serialize, Deserialize, PartialEq, Properties)]
pub struct Game {
    id: String,
    title: String,
    created_at: NaiveDateTime,
    round: u32,
    turn: u32,
    active_team_id: Option<String>,
    active_quest_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    id: String,
    parent_game_id: String,
    no: u32,
    name: String,
    score: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Quest {
    id: String,
    parent_game_id: String,
    no: u32,
    src: String,
    pts: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    id: String,
    parent_game_id: String,
    created_at: NaiveDateTime,
    message: String,
}


// ----- METHODS -----

impl Game {
    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn title(&self) -> String { self.title.clone() }
}
use serde::{Serialize, Deserialize};
use yew::Properties;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct LogEntry {
    id: String,
    parent_game_id: String,
    created_at: NaiveDateTime,
    message: String,
}

impl LogEntry {
    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn created_at(&self) -> NaiveDateTime { self.created_at.clone() }
    pub fn message(&self) -> String { self.message.clone() }
}
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
    //CONSTRUCTORS
    pub fn create(message: &String, game_id: &String, api_key: &String) -> Self {
        todo!()
    }
    pub fn get(id: &String, api_key: &String) -> Self {
        todo!()
    }

    pub fn list(game_id: &String, api_key: &String) -> Vec<Self>{
        todo!()
    }

    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn parent_game_id(&self) -> String { self.parent_game_id.clone() }
    pub fn created_at(&self) -> NaiveDateTime { self.created_at.clone() }
    pub fn message(&self) -> String { self.message.clone() }
}
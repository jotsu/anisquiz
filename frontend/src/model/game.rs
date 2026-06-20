use serde::{Serialize, Deserialize};
use yew::Properties;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub round: u32,
    pub turn: u32,
    pub active_team_id: Option<String>,
    pub active_quest_id: Option<String>,
}

impl Game {
    //CONSTRUCTORS
    pub fn create(title: &String, api_key: &String) -> Self { //TODO: use API
        Game {
            id: "new-game-id".to_string(),
            title: title.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            round: 0,
            turn: 0,
            active_team_id: None,
            active_quest_id: None,
        }
    }
    pub fn get(id: &String, api_key: &String) -> Self { //TODO: use API
        Game {
            id: id.clone(),
            title: "My Game Title".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            round: 0,
            turn: 0,
            active_team_id: None,
            active_quest_id: None,
        }
    }

    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn title(&self) -> String { self.title.clone() }
    pub fn created_at(&self) -> NaiveDateTime { self.created_at.clone() }
    pub fn round(&self) -> u32 { self.round }
    pub fn turn(&self) -> u32 { self.turn }
    pub fn active_team_id(&self) -> Option<String> { self.active_team_id.clone() }
    pub fn active_quest_id(&self) -> Option<String> { self.active_quest_id.clone() }
}
use serde::{Serialize, Deserialize};
use yew::Properties;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Game {
    id: String,
    title: String,
    created_at: NaiveDateTime,
    round: u32,
    turn: u32,
    active_team_id: Option<String>,
    active_quest_id: Option<String>,
}

impl Game {
    //CONSTRUCTORS
    pub fn create(title: &String, api_key: &String) -> Self {
        todo!()
    }
    pub fn get(id: &String, api_key: &String) -> Self {
        todo!()
    }

    pub fn list(api_key: &String) -> Vec<Self>{
        todo!()
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
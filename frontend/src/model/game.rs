use chrono::prelude::*;
use reqwasm::http::Request;
use super::Game;

impl Game {
    //CONSTRUCTORS
    pub fn new() -> Self {
        Game {
            id: "0".to_string(),
            owner_user_id: "0".to_string(),
            title: "New Game".to_string(),
            created_at: chrono::Utc::now(),
            round: 0,
            turn: 0,
            active_team_id: None,
            active_quest_id: None,
        }
    }

    pub async fn create(title: String, api_key: String) -> Self {
        todo!()
    }

    pub async fn get(game_id: String, api_key: String) -> Self {
        Request::get(format!("/g/{}", game_id).as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn list(api_key: String) -> Vec<Self>{
        Request::get(format!("/games").as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    
    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn title(&self) -> String { self.title.clone() }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at.clone() }
    pub fn round(&self) -> u32 { self.round }
    pub fn turn(&self) -> u32 { self.turn }
    pub fn active_team_id(&self) -> Option<String> { self.active_team_id.clone() }
    pub fn active_quest_id(&self) -> Option<String> { self.active_quest_id.clone() }
}
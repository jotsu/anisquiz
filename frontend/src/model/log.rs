use reqwasm::http::Request;
use chrono::prelude::*;
use super::LogEntry;

impl LogEntry {
    //CONSTRUCTORS
    pub async fn create(message: String, game_id: String, api_key: String) -> Self {
        todo!()
    }
    pub async fn get(game_id: String, log_id:String, api_key: String) -> Self {
        Request::get(format!("/g/{}/l/{}", game_id, log_id).as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn list(game_id: String, api_key: String) -> Vec<Self>{
        Request::get(format!("/g/{}/logs", game_id).as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn parent_game_id(&self) -> String { self.parent_game_id.clone() }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at.clone() }
    pub fn message(&self) -> String { self.message.clone() }
}
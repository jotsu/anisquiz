use chrono::prelude::*;
use reqwasm::http::Request;
use crate::{model::*, AppError};

impl GameState {
    pub fn new() -> Self {
        GameState {
            game: Game::new(),
            teams: Vec::<Team>::new(),
            quests: Vec::<Quest>::new(),
            logs: Vec::<LogEntry>::new(),
        }
    }

    pub async fn get(id: &String, token: &String) -> Result<Self, AppError> {
        let game = Game::get(id, token).await?;
        let teams = Team::list(id, token).await?;
        let quests = Quest::list(id, token).await?;
        let logs = LogEntry::list(id, token).await?;
        Ok(GameState{game, teams, quests, logs})
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: "0".to_string(),
            owner_id: "0".to_string(),
            title: "New Game".to_string(),
            created_at: Utc::now(),
            round: 0,
            turn: 0,
            active_team_id: None,
            active_quest_id: None,
        }
    }
    
    pub async fn create(title: String, api_key: String) -> Result::<Self, AppError> {
        todo!()
    }

    pub async fn list(api_key: String) -> Vec<Self>{
        Request::get("/games")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn get(game_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(format!("/g/{}", game_id).as_str())
            .send()
            .await?
            .json()
            .await
            .unwrap()
    }
}


impl Team {
    pub async fn create(game_id: String, api_key: String) -> Self {
        todo!()
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/teams", game_id))
            .send()
            .await?
            .json()
            .await?
    }

    pub async fn get(game_id: &String, team_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/t/{}", game_id, team_id))
            .send()
            .await?
            .json()
            .await?
    }
}

impl Quest {
    pub async fn create(src: &String, game_id: &String, token: &String) -> Result<Self, AppError> {
        todo!()
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/quests", game_id))
            .send()
            .await?
            .json()
            .await?
    }

    pub async fn get(game_id: &String, quest_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/q/{}", game_id, quest_id))
            .send()
            .await?
            .json()
            .await?
    }
}

impl LogEntry {
    pub async fn create(message: &String, game_id: &String, token: &String) -> Result<Self, AppError> {
        todo!()
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/logs", game_id))
            .send()
            .await?
            .json()
            .await?
    }

    pub async fn get(game_id: &String, log_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/l/{}", game_id, log_id))
            .send()
            .await?
            .json()
            .await?
    }
}
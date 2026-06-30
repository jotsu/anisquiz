use chrono::prelude::*;
use reqwasm::http::Request;
use crate::{model::*, AppError};

mod payloads;

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
    
    pub async fn create(title: String, token: &String) -> Result::<Self, AppError> {
        let payload = payloads::CreateGame{title};
        Request::post("/games")
            .body(serde_wasm_bindgen::to_value(&payload)?)
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn list(token: &String) -> Result::<Vec<Self>, AppError>{
        Request::get("/games")
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn get(game_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(format!("/g/{}", game_id).as_str())
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }
}


impl Team {
    pub async fn create(no: u32, name: String, game_id: &String, token: &String) -> Result<Self, AppError> {
        let payload = payloads::CreateTeam{no, name};
        Request::post(format!("/g/{}/teams", game_id).as_str())
            .body(serde_wasm_bindgen::to_value(&payload)?)
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/teams", game_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn get(game_id: &String, team_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/t/{}", game_id, team_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }
}

impl Quest {
    pub async fn create(no: u32, src: String, game_id: &String, token: &String) -> Result<Self, AppError> {
        let payload = payloads::CreateQuest{no, src};
        Request::post(format!("/g/{}/quests", game_id).as_str())
            .body(serde_wasm_bindgen::to_value(&payload)?)
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/quests", game_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn get(game_id: &String, quest_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/q/{}", game_id, quest_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }
}

impl LogEntry {
    pub async fn create(message: String, game_id: &String, token: &String) -> Result<Self, AppError> {
        let payload = payloads::CreateLogEntry{message};
        Request::post(format!("/g/{}/logs", game_id).as_str())
            .body(serde_wasm_bindgen::to_value(&payload)?)
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn list(game_id: &String, token: &String) -> Result<Vec<Self>, AppError> {
        Request::get(&format!("/g/{}/logs", game_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }

    pub async fn get(game_id: &String, log_id: &String, token: &String) -> Result<Self, AppError> {
        Request::get(&format!("/g/{}/l/{}", game_id, log_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send().await?
            .json().await?
    }
}
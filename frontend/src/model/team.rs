use reqwasm::http::Request;
use super::Team;

impl Team {
    //CONSTRUCTORS
    pub async fn create(game_id: String, api_key: String) -> Self {
        todo!()
    }
    pub async fn get(game_id: String, team_id: String, api_key: String) -> Self{
        Request::get(format!("/g/{}/t/{}", game_id, team_id).as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn list(game_id: String, api_key: String) -> Vec<Self> {
        Request::get(format!("/g/{}/teams", game_id).as_str())
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
    pub fn no(&self) -> u32 { self.no }
    pub fn name(&self) -> String { self.name.clone() }
    pub fn score(&self) -> i32 { self.score }
}
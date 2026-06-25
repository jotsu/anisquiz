use reqwasm::http::Request;
use super::Quest;

impl Quest {
    //CONSTRUCTORS
    pub async fn create(src: String, game_id: String, api_key: String) -> Self {
        todo!()
    }
    pub async fn get(game_id: String, quest_id: String, api_key: String) -> Self {
        Request::get(format!("/g/{}/q/{}", game_id, quest_id).as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn list(game_id: String, api_key: String) -> Vec<Self> {
        Request::get(format!("/g/{}/quests", game_id).as_str())
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
    pub fn pts(&self) -> i32 { self.pts }
}
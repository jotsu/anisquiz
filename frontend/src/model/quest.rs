use serde::{Serialize, Deserialize};
use yew::Properties;

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Quest {
    id: String,
    parent_game_id: String,
    no: u32,
    src: String,
    pts: i32,
}

impl Quest {
    //CONSTRUCTORS
    pub fn create(src: &String, game_id: &String, api_key: &String) -> Self {
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
    pub fn no(&self) -> u32 { self.no }
    pub fn pts(&self) -> i32 { self.pts }
}
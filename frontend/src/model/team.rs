use serde::{Serialize, Deserialize};
use yew::Properties;

#[derive(Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Team {
    id: String,
    parent_game_id: String,
    no: u32,
    name: String,
    score: i32,
}

impl Team {
    //CONSTRUCTORS
    pub fn create(game_id: &String, api_key: &String) -> Self {
        todo!()
    }
    pub fn get(id: &String, api_key: &String) -> Self{
        todo!()
    }

    pub fn list(game_id: &String, api_key: &String) -> Vec<Self> {
        todo!()
    }
    
    //GETTERS
    pub fn id(&self) -> String { self.id.clone() }
    pub fn parent_game_id(&self) -> String { self.parent_game_id.clone() }
    pub fn no(&self) -> u32 { self.no }
    pub fn name(&self) -> String { self.name.clone() }
    pub fn score(&self) -> i32 { self.score }
}
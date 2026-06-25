use yew::{Properties, UseStateHandle};
use crate::model::{Game, Team, Quest, LogEntry};

mod game;
mod scoreboard;
mod picker;
mod log;

pub use game::GameComponent;
pub use scoreboard::Scoreboard;
pub use picker::Picker;
pub use log::Log;


#[derive(Clone, PartialEq)]
pub struct GameState {
    pub game: Game,
    pub teams: Vec::<Team>,
    pub quests: Vec::<Quest>,
    pub log: Vec::<LogEntry>,
    pub api_key: String,    //TODO: make it user-specific session token
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub state: UseStateHandle<GameState>,
}
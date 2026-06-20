use yew::{Properties, UseStateHandle};
use crate::model::{Game, Team, Quest, LogEntry};

mod game;
mod scoreboard;
mod picker;
mod log;

pub use game::GameComponent;
// pub use team::TeamComponent;
// pub use quest::QuestComponent;
// pub use log::LogEntryComponent;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub game: UseStateHandle<Game>,
    pub teams: UseStateHandle<Vec::<Team>>,
    pub quests: UseStateHandle<Vec::<Quest>>,
    pub log: UseStateHandle<Vec::<LogEntry>>,
}
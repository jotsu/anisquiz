use yew::prelude::*;

pub mod model;
pub mod components;
use model::{Game, Team, Quest, LogEntry};
use components::{GameState, GameComponent};

fn main() {
    // trunk serve --proxy-backend=http://localhost:3000
    yew::Renderer::<App>::new().render();
}

#[component]
pub fn App() -> Html {

    let game_id = "game_id".to_string();
    let api_key = "api_key".to_string();
    let state = use_state_eq(|| GameState {
        game: Game::get(&game_id, &api_key),
        teams: Team::list(&game_id, &api_key),
        quests: Quest::list(&game_id, &api_key),
        log: LogEntry::list(&game_id, &api_key),
    });

    html!(
        <GameComponent {state} />
    )
}
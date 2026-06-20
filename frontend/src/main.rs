use yew::prelude::*;

pub mod model;
pub mod components;
use model::{Game, Team, Quest, LogEntry};
use components::GameComponent;

fn main() {
    // trunk serve --proxy-backend=http://localhost:3000
    yew::Renderer::<App>::new().render();
}

#[component]
pub fn App() -> Html {


    let id = "game_id".to_string();
    let key = "key".to_string();

    let game = use_state(|| Game::get(&id, &key));
    let teams = use_state(|| Vec::<Team>::new());    //use_state(|| Team::list(&id, &key));
    let quests = use_state(|| Vec::<Quest>::new());    //use_state(|| Team::list(&id, &key));
    let log = use_state(|| Vec::<LogEntry>::new());    //use_state(|| Team::list(&id, &key));

    html!(
        <GameComponent {game} {teams} {quests} {log}/>
    )
}
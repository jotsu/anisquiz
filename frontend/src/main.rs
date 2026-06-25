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
    let loading = use_state(|| true);

    let game_id = "game_id".to_string();
    let api_key = "api_key".to_string();

    let state = use_state_eq(|| GameState {
        game: Game::new(),
        teams: Vec::<Team>::new(),
        quests: Vec::<Quest>::new(),
        log: Vec::<LogEntry>::new(),
        api_key: api_key.clone(),
    });
    
    {
        let state = state.clone();
        let loading = loading.clone();
        loading.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            state.set(GameState {
                game: Game::get(game_id.clone(), api_key.clone()).await,
                teams: Team::list(game_id.clone(), api_key.clone()).await,
                quests: Quest::list(game_id.clone(), api_key.clone()).await,
                log: LogEntry::list(game_id.clone(), api_key.clone()).await,
                api_key: api_key.to_string(),
            });
            loading.set(false);
        });
    }

    if *loading {
        return html! { <p>{ "Loading..." }</p> }
    }

    html!(
        <>
            <nav>
                <h1>{format!("🍋AniSquiz | {}", state.game.title())}</h1>
                <button>{"New Game"}</button>
                <button>{"Load Game"}</button>
            </nav>
            <GameComponent {state} />
        </>
    )
}
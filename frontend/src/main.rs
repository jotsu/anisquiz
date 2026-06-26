use yew::prelude::*;

mod model;
mod api;
mod components;
mod errors;

use model::*;
use errors::AppError;
use components::GameComponent;

fn main() {
    // trunk serve --proxy-backend=http://localhost:3000
    yew::Renderer::<App>::new().render();
}

#[component]
pub fn App() -> Html {
    
    let state= use_state_eq(|| GameState::new());
    let loading = use_state_eq(|| true);
    let error = use_state_eq(|| false);
    
    let token = "my_jwt_token".to_string();
    let game_id = "game_id".to_string();

    {
        let state = state.clone();
        let token = token.clone();
        let game_id = game_id.clone();
        let loading = loading.clone();
        let error = error.clone();
        loading.set(true);
        error.set(false);
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(s) = GameState::get(&game_id, &token).await {
                state.set(s);
                error.set(false);
                loading.set(false);
            } else {
                error.set(true);
                loading.set(false);
            }
        });
    }

    match (*loading, *error) {
        (true, _) => html! { <p>{ "Loading..." }</p> },
        (false, true) => html! { <p>{ "Failed to load game state." }</p> },
        (false, false) => html! { <GameComponent state={state.clone()} token={token.clone()} /> },
    }
}
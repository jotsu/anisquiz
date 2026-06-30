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
    
    let token = use_state_eq(|| None);
    let user_games_list = use_state_eq(|| Vec::<Game>::new());
    let game_state= use_state_eq(|| GameState::new());
    let selected_game_id = use_state_eq(|| None);
    let error = use_state_eq(|| None);
    let loading = use_state_eq(|| true);

    {
        let token = token.clone();
        let user_games_list = user_games_list.clone();
        let selected_game_id = selected_game_id.clone();
        let game_state = game_state.clone();
        let error = error.clone();
        let loading = loading.clone();
        error.set(None);
        loading.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            match (*token, *selected_game_id) {
                (Some(t), Some(g)) => {
                    match GameState::get(g, t).await {
                        Ok(s) => {
                            game_state.set(s);
                            error.set(None);
                            loading.set(false);
                        },
                        Err(e) => {
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                }
                (Some(t), None) => {
                    match Game::list(t).await {
                        Ok(l) => {
                            game_state.set(GameState::new());
                            user_games_list.set(l);
                            error.set(None);
                            loading.set(false);
                        }
                        Err(e) => {
                            game_state.set(GameState::new());
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                },
                (None, _) => {
                    user_games_list.set(Vec::<Game>::new());
                    selected_game_id.set(None);
                    game_state.set(GameState::new());
                    error.set(None);
                    loading.set(false);
                },
            }
        });
    }

    match (*loading, &*error) {
        (true, _,) => html!{
            "Loading..."    //TODO: Loading spinner
        },
        (false, None) => match (*token, *selected_game_id) {
            (Some(t), Some(g)) => html!{
                <GameComponent state={game_state.clone()} token={t.to_string()} />
            },
            (Some(t), None) => html!{
                "Please select or create a new game"    //TODO: Games list
            },
            (None, _) => html!{
                "Please log in or register" //TODO: Login page
            } ,
        },
        (false, Some(e)) => html!{
            format!("Error: {}", e.message) //TODO: Error popup
        },
    }
}
use yew::prelude::*;
use crate::model::*;

#[component]
pub fn GameComponent(props: &Game) -> Html {
    html!(
        <main id={props.id()} class="game">
            // <Nav game_state={game_state.clone()}/>
            // <Scoreboard game_state={game_state.clone()} />
            // <Picker game_state={game_state.clone()} />
        </main>
    )
}
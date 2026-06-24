use yew::prelude::*;
use crate::components::{Props, Scoreboard, Picker, Log};

#[component]
pub fn GameComponent(props: &Props) -> Html {

    html!(
        <main class="game" id={props.state.game.id()}>
            <Scoreboard state={&props.state} />
            <Picker state={&props.state} />
            <Log state={&props.state} />
        </main>
    )
}

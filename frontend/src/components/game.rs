use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn GameComponent(props: &Props) -> Html {

    html!(
        <main class="game" id={props.game.id.clone()}>
            {"Hello"}

        </main>
    )
}

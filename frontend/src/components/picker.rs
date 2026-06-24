use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn Picker(props: &Props) -> Html {
    html!(
        <ul class="picker">
            for quest in &props.state.quests {
                <li class="quest" id={quest.id()}>
                    {quest.no()}
                </li>
            }
        </ul>
    )
}
use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn PickerComponent(props: &Props) -> Html {
    html!(
        <ul class="picker">
            for quest in props.quests.to_vec() {
                <li class="quest" id={quest.id()}>
                    {quest.no()}
                </li>
            }
        </ul>
    )
}
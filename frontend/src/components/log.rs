use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn Log(props: &Props) -> Html {
    html!(
        <ol class="log">
            for entry in &props.state.log {
                <li class="log-entry" id={entry.id()}>
                    {format!("{}: {}", entry.created_at().naive_local(), entry.message())}
                </li>
            }
        </ol>
    )
}
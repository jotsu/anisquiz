use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn LogComponent(props: &Props) -> Html {
    html!(
        <div class="log">
            for entry in props.log.to_vec() {
                <p class="log-entry" id={entry.id()}>
                    {format!("{}: {}", entry.created_at(), entry.message())}
                </p>
            }
        </div>
    )
}
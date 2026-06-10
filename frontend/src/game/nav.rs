use super::*;

#[component]
pub fn Nav(props: &Props) -> Html {
    html!(
        <nav class="nav">
            <h1>{format!("🍋AniSquiz | {}", props.game_state.title.clone())}</h1>
        </nav>
    )
}
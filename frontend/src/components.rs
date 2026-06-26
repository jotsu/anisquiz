use yew::prelude::*;
use crate::model::*;

#[component]
pub fn GameComponent(props: &Props) -> Html {
    html!(
        <main class="game" id={props.state.game.id.clone()}>
            <Scoreboard state={props.state.clone()} token={props.token.clone()}/>
            <Picker state={props.state.clone()} token={props.token.clone()} />
            <Log state={props.state.clone()} token={props.token.clone()} />
        </main>
    )
}

#[component]
pub fn Scoreboard(props: &Props) -> Html {
    html!(
        <table class="scoreboard">
            <th class="no">{"No."}</th>
            <th class="name">{"Name"}</th>
            <th class="score">{"Score"}</th>
            for team in &props.state.teams {
                <tr class="team" id={team.id.clone()}>
                    <td class="no">{team.no}</td>
                    <td class="name">{team.name.clone()}</td>
                    <td class="score">{team.score}</td>
                </tr>
            }
        </table>
    )
}

#[component]
pub fn Picker(props: &Props) -> Html {
    html!(
        <ul class="picker">
            for quest in &props.state.quests {
                <li class="quest" id={quest.id.clone()}>
                    {quest.no}
                </li>
            }
        </ul>
    )
}

#[component]
pub fn Log(props: &Props) -> Html {
    html!(
        <ol class="log">
            for entry in &props.state.logs {
                <li class="log-entry" id={entry.id.clone()}>
                    {format!("{}: {}", entry.created_at.naive_local(), entry.message.clone())}
                </li>
            }
        </ol>
    )
}
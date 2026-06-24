use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn Scoreboard(props: &Props) -> Html {
    html!(
        <table class="scoreboard">
            <th class="no">{"No."}</th>
            <th class="name">{"Name"}</th>
            <th class="score">{"Score"}</th>
            for team in &props.state.teams {
                <tr class="team" id={team.id()}>
                    <td class="no">{team.no()}</td>
                    <td class="name">{team.name()}</td>
                    <td class="score">{team.score()}</td>
                </tr>
            }
        </table>
    )
}
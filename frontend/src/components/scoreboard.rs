use yew::prelude::*;
use crate::components::Props;

#[component]
pub fn ScoreboardComponent(props: &Props) -> Html {
    html!(
        <table class="scoreboard">
            for team in props.teams.to_vec() {
                <tr class="team" id={team.id()}>
                    //TODO
                </tr>
            }
        </table>
    )
}
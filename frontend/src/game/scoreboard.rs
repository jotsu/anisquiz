use super::*;

#[component]
pub fn Scoreboard(props: &Props) -> Html {
    let add_team = |name: String| {
        let gs = props.game_state.clone();
        Callback::from(move |_| { gs.add_team(name.clone()) })
    };
    let delete_team = |id: String| {
        let gs = props.game_state.clone();
        Callback::from(move |_| { gs.delete_team(id.clone()) })
    };
    let next_team = |_| {
        let gs = props.game_state.clone();
        Callback::from(move |_| { gs.next_team() })
    };
    let change_name = |team: Team| {
        let gs = props.game_state.clone();
        Callback::from(move |e: Event| {
            let input = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            let mut team = team.clone();
            if let Some(input) = input {
                team.name = input.value();
                gs.update_team(team);
            }
        })
    };
    let change_score = |team: Team| {
        let gs = props.game_state.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            && let Some(value) = input.value().parse().ok() {
                let mut team = team.clone();
                team.score = value;
                gs.update_team(team);
            }
        })
    };
    
    html!(
        <div class="scoreboard">
            <h2>{ "Scoreboard" }</h2>
            <button onclick={add_team("New Team".to_string())}>{"➕"}</button>
            <button onclick={next_team(())}>{"⏭️"}</button>
            <table id="scoreboard">
                <tr class="headers">
                    <th class="no">{ "No." }</th>
                    <th class="name">{ "Name" }</th>
                    <th class="score">{ "Score" }</th>
                    <th class="action">{ "Action" }</th>
                </tr>
                for team in &props.game_state.get_teams() {
                    // <TeamComponent team={team.clone()} state={props.state.clone()} />
                    <tr id={team.id.to_string()} class={
                        if let Some(at) = props.game_state.get_active_team() && team.id == at.id
                        { "active team" } else { "team" }} >
                        <td class="no">{team.no}</td>
                        <td class="name"><input type="text" onchange={change_name(team.clone())} value={team.name.to_string()} /></td>
                        <td class="score"><input type="number" step="1" onchange={change_score(team.clone())} value={team.score.to_string()} /></td>
                        <td class="action">
                            <button onclick={delete_team(team.id.to_string())}>{"❌"}</button>
                        </td>
                    </tr>
                }
            </table>
        </div>
    )
}

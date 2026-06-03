use super::*;

#[component]
pub fn Picker(props: &Props) -> Html {
    let quests = props.game_state.quests.to_vec();

    html!(
        <div class="picker">
            <h2>{ "Picker" }</h2>
            <ul id="picker">
                for quest in quests {
                    <li class="quest" id={quest.id.to_string()}>
                        {quest.no}
                        <br/>
                        <span class="pts">
                            for _ in 0..quest.pts {
                                {"⭐"}
                            }
                        </span>
                    </li>
                }
            </ul>
        </div>
    )
}

use yew:: prelude::*;
use uuid::Uuid;
use web_sys::wasm_bindgen::JsCast;

mod game;

#[component]
pub fn App() -> Html {
    //TODOS
    let game_id = Uuid::new_v4().to_string();
    let _timer = |_: ()| todo!();
    let _show_answer = |_: ()| todo!();
    let _gamelog = |_: ()| todo!();
    let _save_game = |_: ()|todo!();
    let _load_game = |_: ()|todo!();
    let _upload_screenshots = |_: ()| todo!();
    let _load_screenshots = |_: ()| todo!();

    let active_team = use_state_eq(|| None);
    let selected_screenshot = use_state_eq(|| None);
    // GAME STATE
    let teams = use_state_eq(|| {
        vec![Team {
                id: Uuid::new_v4().to_string().into(),
                no: 1,
                name: "New Team".into(),
                score: 0,
                answers: Vec::<(Screenshot, bool)>::new(),
            }]
    });


    let screenshots = use_state_eq(|| {
        let mut init_screenshots = Vec::<Screenshot>::new();
        for i in 0..100 {
            init_screenshots.push(
                Screenshot {
                    id: Uuid::new_v4().to_string().into(),
                    no: i+1,
                    src: format!("screenshots/{:03}.jpg", i+1).into(),
                    pts: 3,
                    status: ScreenshotStatus::New,
                    picked_by: Vec::<(AttrValue, AttrValue)>::new(),
                    answered_by: None,
                }
            )
        }
        init_screenshots
    });
    

    // FUNCTIONS / CALLBACKS
    let add_team = {
        let teams = teams.clone();
        let active_team = active_team.clone();
        Callback::from(move |new_name: AttrValue| {
            let mut new_teams = teams.to_vec();
            let new_team = Team {
                    id: Uuid::new_v4().to_string().into(),
                    no: new_teams.len()+1,
                    name: new_name,
                    score: 0,
                    answers: Vec::<(Screenshot, bool)>::new(),
                };
            new_teams.push(new_team.clone());
            teams.set(new_teams);
            if active_team.is_none() {
                active_team.set(Some(new_team));
            }
        })
    };
    let team_op = {
        let teams = teams.clone();
        let active_team = active_team.clone();
        Callback::from(move |(id, op): (AttrValue, TeamOp)|{
            if let Some(idx) = teams.iter().position(|t| t.id == id) {
                let mut new_teams = teams.to_vec();
                match op {
                    TeamOp::MoveUp => {
                        if idx > 0 {
                            let t = new_teams.remove(idx);
                            new_teams.insert(idx - 1, t);
                            for i in 0..new_teams.len(){
                                new_teams[i].no = i+1;
                            }
                        }
                    },
                    TeamOp::MoveDown => {
                        if idx < teams.len() - 1 {
                            let t = new_teams.remove(idx);
                            new_teams.insert(idx + 1, t);
                            for i in 0..new_teams.len(){
                                new_teams[i].no = i+1;
                            }
                        }
                    },
                    TeamOp::SetName(new_name) => new_teams[idx].name = new_name.unwrap_or(new_teams[idx].name.clone()),
                    TeamOp::SetScore(new_score) => new_teams[idx].score = new_score.unwrap_or(new_teams[idx].score),
                    TeamOp::SetActive => active_team.set(Some(new_teams[idx].clone())),
                    TeamOp::Delete(confirmation) => {
                        if confirmation {
                            if let Some(team) = &*active_team && team.id == id {
                                active_team.set(
                                    if idx+1 < new_teams.len() {
                                        Some(new_teams[idx+1].clone())
                                    } else if new_teams.len() > 1{
                                        Some(new_teams[0].clone())
                                    } else {
                                        None
                                    }
                                );
                            }
                            new_teams.remove(idx);
                            for i in 0..new_teams.len(){
                                new_teams[i].no = i+1;
                            }
                        }
                    },
                }
                if let (Some(at), Some(st)) = (&*active_team, new_teams.iter().find(|t| t.id == id)) && at.id == st.id {
                    active_team.set(Some(st.clone()));
                }
                teams.set(new_teams);
            }
        })
    };

    let screenshot_op = {
        let screenshots = screenshots.clone();
        let selected_screenshot = selected_screenshot.clone();
        let teams = teams.clone();
        let active_team = active_team.clone();
        Callback::from(move |(id, op): (AttrValue, ScreenshotOp)|{
            match op {
                ScreenshotOp::Open => {
                    if active_team.is_some() {
                        selected_screenshot.set(screenshots.to_vec().into_iter().find(|s| s.id == id))
                    }
                },
                ScreenshotOp::Close => selected_screenshot.set(None),
                ScreenshotOp::Answer(is_correct) => {
                    if let (Some(ss), Some(at)) = (&*selected_screenshot, &*active_team) && ss.id == id {
                        let mut new_screenshots = screenshots.to_vec();
                        let mut new_teams = teams.to_vec();
                        if let (Some(s_idx), Some(t_idx)) = (new_screenshots.iter().position(|s| s.id == id), new_teams.iter().position(|t| t.id == at.id)) {
                            let screenshot = &mut new_screenshots[s_idx];
                            let team = &mut new_teams[t_idx];
                            if screenshot.status != ScreenshotStatus::Answered && screenshot.status != ScreenshotStatus::Failed {
                                team.answers.push((screenshot.clone(), is_correct));
                                screenshot.picked_by.push((team.id.clone(), team.name.clone()));
                                if is_correct {
                                    team.score += screenshot.pts;
                                    screenshot.status = ScreenshotStatus::Answered;
                                    screenshot.answered_by = Some((team.id.clone(), team.name.clone()));
                                } else {
                                    screenshot.pts += -1;
                                    screenshot.status = {
                                        if screenshot.pts > 0 {
                                            ScreenshotStatus::Picked
                                        } else {
                                            ScreenshotStatus::Failed
                                        }
                                    };
                                }
                                selected_screenshot.set(None);
                                active_team.set(
                                    if t_idx+1 < new_teams.len() {
                                        Some(new_teams[t_idx+1].clone())
                                    } else if new_teams.len() > 0 {
                                        Some(new_teams[0].clone())
                                    } else {
                                        None
                                    }
                                );
                                screenshots.set(new_screenshots);
                                teams.set(new_teams);
                            }
                        }
                    }
                },
            }
        })
    };

    if let Some(ss) = &*selected_screenshot {
        html! {
            <ScreenshotModal game_id={game_id.clone()} id={ss.id.clone()} no={ss.no} src={ss.src.clone()} pts={ss.pts} status={ss.status.clone()} team={(*active_team).clone()} cb={screenshot_op.clone()}/>
        }
    } else {
        html! {<>
            <nav>
                <h1>{ "🍋 AniSquiz" }</h1>
                <div class="game-info">
                    {format!("G: {}", &game_id.to_string())}
                    <br/>{format!("T: {}", if let Some(t) = &*active_team {t.id.to_string()} else {"None".to_string()})}
                    <br/>{format!("S: {}",if let Some(s) = &*selected_screenshot {s.id.to_string()} else {"None".to_string()})}
                </div>
            </nav>
    
            <main>
                // SCOREBOARD
                <h3>{ "Scoreboard" }</h3>
                <button onclick={move |_| add_team.emit("New Team".into())}>{"➕"}</button>
                <button >{"🔀"}</button>
                <table id="scoreboard">
                    <tr class="headers">
                        <th class="no">{ "No." }</th>
                        <th class="name">{ "Name" }</th>
                        <th class="score">{ "Score" }</th>
                        <th class="action">{ "Action" }</th>
                    </tr>
                for Team { id, no, name, score, .. } in teams.to_vec() {
                    <TeamTR game_id={game_id.clone()} id={id.clone()} {no} name={name.clone()} {score} active={
                        if let Some(team) = &*active_team && team.id == id.clone() {
                            true
                        } else {false}
                    }  cb={team_op.clone()}/>
                }
                </table>
    
                // PICKER
                <h3>{ "Picker" }</h3>
                <ul id="picker">
                    for Screenshot { id, no, src, pts, status, ..} in screenshots.to_vec() {
                        <ScreenshotLI game_id={game_id.clone()} id={id.clone()} {no} src={src.clone()} {pts} status={status.clone()} team={(*active_team).clone()} cb={screenshot_op.clone()} />
                    }
                </ul>
    
            </main>
        </>}
    }

}

//TEAM
#[derive(Clone, PartialEq)]
struct Team {
    id: AttrValue,
    no: usize,
    name: AttrValue,
    score: i32,
    answers: Vec<(Screenshot, bool)>,
}
#[derive(Clone, PartialEq)]
enum TeamOp {
    MoveUp,
    MoveDown,
    SetName(Option<AttrValue>),
    SetScore(Option<i32>),
    SetActive,
    Delete(bool),
}
#[derive(Properties, Clone, PartialEq)]
struct TeamProps {
    game_id: String,
    id: AttrValue, no: usize, name: AttrValue, score: i32, active: bool,
    cb: Callback<(AttrValue, TeamOp)>
}
#[component]
fn TeamTR(props: &TeamProps) -> Html {
    let on_click = |op: TeamOp| {
        let cb = props.cb.clone();
        let id = props.id.clone();
        Callback::from(move |_|{
            let id = id.clone();
            let op = op.clone();
            cb.emit((id, op))
        })
    };

    let on_change = |op: TeamOp| {
        let cb = props.cb.clone();
        let id = props.id.clone();

        Callback::from(move |e: Event| {            
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target= e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            if let Some(input) = input {
                match op {
                    TeamOp::SetName(_) => cb.emit((id.clone(), TeamOp::SetName(Some(input.value().into())))),
                    TeamOp::SetScore(_) => cb.emit((id.clone(), TeamOp::SetScore(input.value().parse().ok()))),
                    _ => ()
                }
            }
        })
    };

    html!(
        <tr id={props.id.to_string()} class={format!("team {}", if props.active {"active"} else {""})} >
            <td class="no">{props.no}</td>
            <td class="name" ><input type="text" onchange={on_change(TeamOp::SetName(None))} value={props.name.to_string()} /></td>
            <td class="score"><input type="number" step="1" onchange={on_change(TeamOp::SetScore(None))} value={props.score.to_string()} /></td>
            <td class="actions">
                <button onclick={on_click(TeamOp::SetActive)}>{"⏯️"}</button>
                <button onclick={on_click(TeamOp::MoveUp)}>{"🔼"}</button>
                <button onclick={on_click(TeamOp::MoveDown)}>{"🔽"}</button>
                <button onclick={on_click(TeamOp::Delete(true))}>{"❌"}</button>
            </td>
        </tr>
    )
}


// SCREENSHOT
#[derive(Clone, PartialEq)]
struct Screenshot {
    id: AttrValue,
    no: usize,
    src: AttrValue,
    pts: i32,
    status: ScreenshotStatus,
    picked_by: Vec<(AttrValue, AttrValue)>,
    answered_by: Option<(AttrValue, AttrValue)>,
}
#[derive(Clone, PartialEq)]
enum ScreenshotStatus {
    New,
    Picked,
    Answered,
    Failed,
}
#[derive(Clone, PartialEq)]
enum ScreenshotOp {
    Open,
    Close,
    Answer(bool),
}
#[derive(Properties, Clone, PartialEq)]
pub struct ScreenshotProps {
    game_id: String,
    id: AttrValue, no: usize, src: AttrValue, pts: i32, status: ScreenshotStatus,
    team: Option<Team>,
    cb: Callback<(AttrValue, ScreenshotOp)>
}

#[component]
fn ScreenshotLI(props: &ScreenshotProps) -> Html {
    let action = |op: ScreenshotOp|{
        let cb = props.cb.clone();
        let id = props.id.clone();
        Callback::from(move |_| {
            let id = id.clone();
            let op = op.clone();
            cb.emit((id, op))
        })
    };
    
    html!(
            <li id={props.id.to_string()}
                class={format!("screenshot {}", match props.status {
                    ScreenshotStatus::New => "screenshot new",
                    ScreenshotStatus::Picked => "screenshot picked",
                    ScreenshotStatus::Answered => "screenshot answered",
                    ScreenshotStatus::Failed => "screenshot failed",
                    })
                }
                onclick={action(ScreenshotOp::Open)}>
                { format!("{:03}", props.no) }
                <br/>
                <sup class="pts">for _i in 0..props.pts {{"⭐"}}</sup>
                
            </li>
    )
}

#[component]
fn ScreenshotModal(props: &ScreenshotProps) -> Html {
    let action = |op: ScreenshotOp|{
        let cb = props.cb.clone();
        let id = props.id.clone();
        Callback::from(move |_| {
            let id = id.clone();
            let op = op.clone();
            cb.emit((id, op))
        })
    };

    html!(
        <div class="modal">
            <div class="close" onclick={action(ScreenshotOp::Close)}>{"❌"}</div>
            if let Some(t) = &props.team {
                <div class="modal-info">
                    <div class="game-info">
                        {format!("G: {}  ", &props.game_id.to_string())}
                        <br/>{format!("T: {}  ", if let Some(t) = &props.team {t.id.to_string()} else {"None".to_string()})}
                        <br/>{format!("S: {}", props.id.to_string())}
                    </div>
                    <div class="ss-info">
                        <strong>{format!("▸ {:03}", props.no)}</strong>
                        <sup class="pts">for _i in 0..props.pts {{"⭐"}}</sup>
                        <br/>
                        <strong>{format!("▸ {} ", t.name)}</strong>{format!("({}P)", t.score)}
                        <br/>
                        <button class="right" onclick={action(ScreenshotOp::Answer(true))}>{"✓"}</button>
                        <button class="wrong" onclick={action(ScreenshotOp::Answer(false))}>{"✗"}</button>
                    </div>
                </div>
            }
            <img class="modal-img" src={props.src.to_string()} />
        </div>
    )
}

#[component]
fn PopupPrompt() -> Html {
    html!(
        <div class="overlay">
            <div class="popup">

            </div>
        </div>
    )
}
use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use nanoid::nanoid;

pub mod nav;
pub mod picker;
pub mod scoreboard;

use nav::*;
use picker::*;
use scoreboard::*;

#[derive(Clone, PartialEq)]
pub struct Team {
    id: String,
    no: usize,
    name: String,
    score: i32,
}

#[derive(Clone, PartialEq)]
pub struct Quest {
    id: String,
    no: usize,
    src: String,
    pts: i32,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub round: usize,
    pub turn: usize,
    pub teams: Vec<Team>,
    pub quests: Vec<Quest>,
    pub active_team: Option<Team>,
    pub active_quest: Option<Quest>,
    pub log: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    id: String,
    title: String,
    round: UseStateHandle<usize>,
    turn: UseStateHandle<usize>,
    teams: UseStateHandle<Vec<Team>>,
    quests: UseStateHandle<Vec<Quest>>,
    active_team: UseStateHandle<Option<Team>>,
    active_quest: UseStateHandle<Option<Quest>>,
    log: UseStateHandle<Vec<String>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub game_state: GameState,
}

#[component]
pub fn GameComponent(props: &Game) -> Html {
    let game_state = GameState {
        id: props.id.clone(),
        title: props.title.clone(),
        round: use_state_eq(|| 0),
        turn: use_state_eq(|| 0),
        teams: use_state_eq(|| props.teams.clone()),
        quests: use_state_eq(|| props.quests.clone()),
        active_team: use_state_eq(|| if props.teams.len() > 0 {Some(props.teams.first().unwrap().clone())} else {None}),
        active_quest: use_state_eq(|| if props.quests.len() > 0 {Some(props.quests.first().unwrap().clone())} else {None}),
        log: use_state_eq(|| props.log.clone())
    };

    html!(
        <main id={game_state.id.clone()} class="game">
            <Nav game_state={game_state.clone()}/>
            <Scoreboard game_state={game_state.clone()} />
            <Picker game_state={game_state.clone()} />
        </main>
    )
}

impl From<&GameState> for Game {
    fn from(game_state: &GameState) -> Self {
        Game {
            id: game_state.get_id(),
            title: game_state.get_title(),
            round: game_state.get_round(),
            turn: game_state.get_turn(),
            teams: game_state.get_teams(),
            quests: game_state.get_quests(),
            active_team: game_state.get_active_team(),
            active_quest: game_state.get_active_quest(),
            log: game_state.get_log(),
        }
    }
}

impl GameState {
    // GETTERS
    fn get_id(&self) -> String { self.id.clone() }
    fn get_title(&self) -> String { self.title.clone() }
    fn get_round(&self) -> usize { *self.round }
    fn get_turn(&self) -> usize { *self.turn }
    fn get_teams(&self) -> Vec<Team> { self.teams.to_vec() }
    fn get_quests(&self) -> Vec<Quest> { self.quests.to_vec() }
    fn get_active_team(&self) -> Option<Team> {
        if let Some(t) = &*self.active_team { Some(t.clone()) } else { None } }
    fn get_active_quest(&self) -> Option<Quest> {
        if let Some(q) = &*self.active_quest { Some(q.clone()) } else { None } }
    fn get_log(&self) -> Vec<String> { self.log.to_vec() }

    //SETTERS
    fn set_id(&mut self, id: String) -> () { self.id = id }
    fn set_title(&mut self, title: String) -> () { self.title = title }
    fn set_round(&self, round: usize) -> () { self.round.set(round) }
    fn set_turn(&self, turn: usize) -> () { self.turn.set(turn) }
    fn set_teams(&self, teams: Vec<Team>) -> () { self.teams.set(teams) }
    fn set_quests(&self, quests: Vec<Quest>) -> () { self.quests.set(quests) }
    fn set_active_team(&self, active_team: Option<Team>) -> () { self.active_team.set(active_team) }
    fn set_active_quest(&self, active_quest: Option<Quest>) -> () { self.active_quest.set(active_quest) }
    fn set_log(&self, log: Vec<String>) { self.log.set(log) }

    fn add_team(&self, team_name: String) -> () {
        let new_team = Team { id: nanoid!(10), no: self.teams.len()+1, name: team_name, score: 0 };
        let mut new_teams = self.get_teams();
        new_teams.push(new_team.clone());
        self.teams.set(new_teams);
        if self.teams.len() == 1 {
            self.active_team.set(Some(new_team.clone()));
        }
    }

    fn update_team(&self, new_team: Team) -> (){
        if let Some(idx) = self.get_teams().iter().position(|t| t.id == new_team.id){
            let mut new_teams = self.get_teams();
            new_teams.remove(idx);
            new_teams.insert(idx, new_team.clone());
            self.teams.set(new_teams);
            if let Some(at) = self.get_active_team() && at.id == new_team.id {
                self.active_team.set(Some(new_team.clone()));
            }
        }
    }

    fn delete_team(&self, team_id: String) -> () {
        let mut new_teams = self.teams.to_vec();
        if let Some(idx) = new_teams.iter().position(|t| t.id == team_id) {
            new_teams.remove(idx);
            for i in idx..new_teams.len() {
                new_teams[i].no = i+1;
            }
            if let Some(at) = &*self.active_team && self.teams[idx].id == at.id { self.next_team() }
        }
        self.teams.set(new_teams);
    }

    fn next_team(&self) -> () {
        if self.teams.len() > 0 {
            if let Some(a_team) = &*self.active_team
                && let Some(idx) = self.teams.to_vec().iter().position(|t| t.id == a_team.id)
                && idx + 1 < self.teams.len() {
                    self.active_team.set(Some(self.teams[idx+1].clone()));
            } else {
                self.active_team.set(Some(self.teams[0].clone()));
            }
        } else {
            self.active_team.set(None)
        }
    }

}
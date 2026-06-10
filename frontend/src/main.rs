use yew:: prelude::*;
use nanoid::nanoid;

pub mod game;
use game::*;

#[component]
pub fn App() -> Html {
    let game = Game {
        id: nanoid!(10),
        title: "Animcowa Memówka".to_string(),
        teams: Vec::<Team>::new(),
        quests: Vec::<Quest>::new(),
        round: 1,
        turn: 1,
        active_team: None,
        active_quest: None,
        log: Vec::<String>::new(),
    };

    html!(
        <GameComponent
            id={game.id}
            title={game.title}
            teams={game.teams}
            quests={game.quests}
            round={game.round}
            turn={game.turn}
            active_team={game.active_team}
            active_quest={game.active_quest}
            log={game.log}
        />
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
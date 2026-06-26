use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGame {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTeam {
    pub no: u32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateQuest {
    pub no: u32,
    pub src: String,
}

#[derive(Deserialize)]
pub struct CreateLogEntry {
    pub message: String,
}
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateGame {
    pub title: String,
}

#[derive(Serialize)]
pub struct CreateTeam {
    pub no: u32,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateQuest {
    pub no: u32,
    pub src: String,
}

#[derive(Serialize)]
pub struct CreateLogEntry {
    pub message: String,
}
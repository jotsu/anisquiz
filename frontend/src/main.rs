use yew::prelude::*;

pub mod model;
mod components;

fn main() {
    // trunk serve --proxy-backend=http://localhost:3000
    yew::Renderer::<App>::new().render();
}

#[component]
pub fn App() -> Html {
    html!("Welcome, cyber traveler!")
}
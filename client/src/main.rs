use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={stylesheet}>
        <h1>{"Nazdárek!"}</h1>
        <h2>{"Ahojky!"}</h2>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
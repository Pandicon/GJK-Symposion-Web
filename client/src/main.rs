mod components;
mod pages;
pub mod router;
pub mod types;
pub mod utils;

use crate::router::{switch, Route};

use yew::prelude::*;
use yew_router::prelude::*;

const MOSTY_TEXT: &str = include_str!("../mosty_text.txt");

#[function_component(App)]
fn app() -> Html {
	gloo::console::log!(MOSTY_TEXT);
	html! {
		<BrowserRouter>
			<Switch<Route> render={Switch::render(switch)} />
		</BrowserRouter>
	}
}

fn main() {
	yew::start_app::<App>();
}

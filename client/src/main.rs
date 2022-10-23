mod pages;
mod router;
pub mod utils;

use crate::router::{switch, Route};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
	html! {
		<BrowserRouter>
			<Switch<Route> render={Switch::render(switch)} />
		</BrowserRouter>
	}
}

fn main() {
	yew::start_app::<App>();
}

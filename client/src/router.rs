use crate::{
	pages::{chyba::Chyba, harmonogram::Harmonogram, home::Home, kontakty::Kontakty, not_found::NotFound, o_akci::OAkci},
	types,
};

use yew::prelude::*;
use yew_router::prelude::*;

const CONFIG_TEXT: &str = include_str!("../config.json");

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/harmonogram/:day")]
	Harmonogram { day: String },
	#[at("/harmonogram")]
	HarmonogramAll,
	#[at("/kontakty")]
	Kontakty,
	#[not_found]
	#[at("/404")]
	NotFound,
	#[at("/o_akci")]
	OAkci,
}

pub fn switch(route: &Route) -> Html {
	let config: types::Config = match serde_json::from_str(CONFIG_TEXT) {
		Ok(data) => data,
		Err(error) => {
			let error_message = format!("Failed to parse the config file, can not continue: {}", error);
			gloo::console::error!(&error_message);
			return html! { <Chyba error = {error_message} /> };
			// TODO Add an error message parameter
		}
	};
	match route {
		Route::Home => html! { <Home /> },
		Route::Harmonogram { day } => html! { <Harmonogram day={Some(day.to_owned())} config={config} />},
		Route::HarmonogramAll => html! { <Harmonogram config={config} />},
		Route::Kontakty => html! { <Kontakty />},
		Route::NotFound => html! { <NotFound /> },
		Route::OAkci => html! { <OAkci /> },
	}
}

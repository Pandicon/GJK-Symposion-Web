use crate::{
	pages::{harmonogram::Harmonogram, home::Home, kontakty::Kontakty, not_found::NotFound, o_akci::OAkci},
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
	#[at("/chyba")]
	FrontEndError,
	#[at("/o_akci")]
	OAkci,
}

pub fn switch(route: &Route) -> Html {
	let config: types::Config = if *route != Route::FrontEndError {
		match serde_json::from_str(CONFIG_TEXT) {
			Ok(data) => data,
			Err(error) => {
				gloo::console::error!(format!("Failed to parse the config file, can not continue: {}", error));
				return html! { <Redirect<Route> to={Route::FrontEndError}/> }; // TODO Add an error message parameter
			}
		}
	} else {
		types::Config { api: String::new() }
	};
	match route {
		Route::Home => html! { <Home /> },
		Route::Harmonogram { day } => html! { <Harmonogram day={Some(day.to_owned())} config={config} />},
		Route::HarmonogramAll => html! { <Harmonogram config={config} />},
		Route::Kontakty => html! { <Kontakty />},
		Route::NotFound => html! { <NotFound /> },
		Route::FrontEndError => html! {<h1>{"Ajajaj"}</h1>},
		Route::OAkci => html! { <OAkci /> },
	}
}

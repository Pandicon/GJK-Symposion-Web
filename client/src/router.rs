use crate::{
	pages::{chyba::Chyba, clear_storage::ClearStorage, harmonogram::Harmonogram, home::Home, kontakty::Kontakty, not_found::NotFound, o_akci::OAkci},
	types,
};

use yew::prelude::*;
use yew_router::prelude::*;

const CONFIG_TEXT: &str = include_str!("../config.json");

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/clear_storage")]
	ClearStorage,
	#[at("/harmonogram")]
	HarmonogramAll,
	#[at("/harmonogram/details/:id")]
	HarmonogramAllDetails { id: String },
	#[at("/harmonogram/:day")]
	Harmonogram { day: String },
	#[at("/harmonogram/:day/details/:id")]
	HarmonogramDetails { day: String, id: String },
	#[at("/kontakty")]
	Kontakty,
	#[at("/nazdarek")]
	Nazdarek,
	#[at("/o_akci")]
	OAkci,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(route: &Route) -> Html {
	let config: types::Config = match serde_json::from_str(CONFIG_TEXT) {
		Ok(data) => data,
		Err(error) => {
			let error_message = format!("Failed to parse the config file, can not continue: {}", error);
			gloo::console::error!(&error_message);
			return html! { <Chyba error = {error_message} /> };
		}
	};
	match route {
		Route::ClearStorage => html! { <ClearStorage /> },
		Route::Home => html! { <Home config={config} /> },
		Route::Harmonogram { day } => html! { <Harmonogram day={Some(day.to_owned())} config={config} />},
		Route::HarmonogramDetails { day, id } => html! { <Harmonogram day={Some(day.to_owned())} config={config} details_id={id.clone()} />},
		Route::HarmonogramAll => html! { <Harmonogram config={config} />},
		Route::HarmonogramAllDetails { id } => html! { <Harmonogram config={config} details_id={id.clone()} /> },
		Route::Kontakty => html! { <Kontakty />},
		Route::Nazdarek => html! { <><h1>{"Nazdárek!"}</h1><script>{r#"window.location.href = "http://gjk.cz/~kupka/";"#}</script></> },
		Route::NotFound => html! { <NotFound /> },
		Route::OAkci => html! { <OAkci /> },
	}
}

use crate::pages::{harmonogram::Harmonogram, home::Home, kontakty::Kontakty, not_found::NotFound, o_akci::OAkci};

use yew::prelude::*;
use yew_router::prelude::*;

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
	match route {
		Route::Home => html! { <Home /> },
		Route::Harmonogram { day } => html! { <Harmonogram day={Some(day.to_owned())} />},
		Route::HarmonogramAll => html! { <Harmonogram />},
		Route::Kontakty => html! { <Kontakty />},
		Route::NotFound => html! { <NotFound /> },
		Route::OAkci => html! { <OAkci /> },
	}
}

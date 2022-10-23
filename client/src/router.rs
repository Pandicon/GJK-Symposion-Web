use crate::pages::{home::Home, harmonogram::Harmonogram, not_found::NotFound};

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
	#[not_found]
	#[at("/404")]
	NotFound
}

pub fn switch(route: &Route) -> Html {
	match route {
		Route::Home => html!{ <Home /> },
		Route::Harmonogram { day } => html!{ <Harmonogram day={Some(day.to_owned())} />},
		Route::HarmonogramAll => html!{ <Harmonogram />},
		Route::NotFound => html! { <NotFound /> }
	}
}
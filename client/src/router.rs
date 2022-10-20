use crate::pages::{home::Home, harmonogram::Harmonogram};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/harmonogram")]
	Harmonogram
}

pub fn switch(route: &Route) -> Html {
	match route {
		Route::Home => html!{ <Home /> },
		Route::Harmonogram => html!{ <Harmonogram /> }
	}
}
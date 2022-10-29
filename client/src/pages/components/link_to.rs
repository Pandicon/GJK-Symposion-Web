use yew::prelude::*;

use crate::router::Route;

use yew_router::history::History;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	pub route: Route,
	pub path: String,
	pub children: Option<Children>,
	pub a_style: Option<String>,
	pub div_style: Option<String>,
}

#[function_component(LinkTo)]
pub fn home(props: &Props) -> Html {
	html! {
		<>
		if let Some(history) = yew_router::hooks::use_history() {
			<div onclick={
				let route = props.route.clone();
				Callback::from(move |_| {
					history.push(route.clone());
				})
			} style={props.div_style.clone()}>
			if let Some(children) = props.children.clone() {
				{ children }
			}
			</div>
		} else {
			<a href={props.path.clone()} style={props.a_style.clone()}>
			if let Some(children) = props.children.clone() {
				{ children }
			}
			</a>
		}
		</>
	}
}

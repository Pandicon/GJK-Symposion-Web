use yew::prelude::*;

use crate::router::Route;

use yew_router::history::History;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	/// The router route to which to redirect to
	pub route: Route,
	/// The path to redirect to in case it is impossible to use the browser history - should usually correspond to the route chosen
	pub path: String,
	/// The children of this component
	pub children: Option<Children>,
	/// The styles to apply in case it is a normal `<a>` link
	pub link_style: Option<String>,
	/// The styles to apply in case it uses browser history so the `<a>` tag has no href attribute
	pub history_style: Option<String>,
}

/// # The LinkTo component
/// This component tries to use browser history for redirects if possible. This way the page doesn't reload and doesn't have to fetch all of the uncached files again, which saves load times.
/// If browser history is available, it uses the route specified to redirect the user, else it uses the path and acts as a normal link.
#[function_component(LinkTo)]
pub fn link_to(props: &Props) -> Html {
	html! {
		<>
		if let Some(history) = yew_router::hooks::use_history() {
			<a onclick={
				let route = props.route.clone();
				Callback::from(move |_| {
					history.push(route.clone());
				})
			} style={props.history_style.clone()}>
			if let Some(children) = props.children.clone() {
				{ children }
			}
			</a>
		} else {
			<a href={props.path.clone()} style={props.link_style.clone()}>
			if let Some(children) = props.children.clone() {
				{ children }
			}
			</a>
		}
		</>
	}
}

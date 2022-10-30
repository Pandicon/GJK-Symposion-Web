use yew::prelude::*;

/// The #NotFound page
/// This is the 404 not found page that will show up when you go to a link that doesn't exist
#[function_component(NotFound)]
pub fn not_found() -> Html {
	yew_hooks::use_title("404 | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<h1>{"404"}</h1>
		<h2>{"Most do této destinace spadl"}</h2>
		</>
	}
}

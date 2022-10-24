use yew::prelude::*;

#[function_component(NotFound)]
pub fn harmonogram() -> Html {
	yew_hooks::use_title("404 | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<h1>{"404"}</h1>
		<h2>{"Most do této destinace spadl"}</h2>
		</>
	}
}

use crate::{pages::components::link_to::LinkTo, router::Route};
use yew::prelude::*;

#[function_component(OAkci)]
pub fn home() -> Html {
	yew_hooks::use_title("O Akci | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<header class="generic_header">
			<h1>
			<LinkTo path="/" route={Route::Home} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
				<span class="most">{"MOSTY"}</span>
			</LinkTo>
			</h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2><span class="most">{"Informace"}</span></h2>
			</div>
		</header>
		<main>
			<div class="opakujici_most"></div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

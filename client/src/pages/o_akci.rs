use yew::prelude::*;

#[function_component(OAkci)]
pub fn home() -> Html {
	html! {
		<>
		<header>
			<h1><a href="/">{"MOSTY"}</a></h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2>{"Informace"}</h2>
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

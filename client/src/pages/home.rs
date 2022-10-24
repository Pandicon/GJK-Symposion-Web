use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
	html! {
		<>
		<header>
			<div class="title">
				<div class="first_line">
					<h1><a href="/">{"MOSTY"}</a></h1>
					<div class="opakujici_most"></div>
				</div>
				<h2>{"SYMPOSION GYMNÁZIA JANA KEPLERA"}</h2>
			</div>
			<div class={"date"}>
				<h2>{"16. - 18. listopadu 2022"}</h2>
				<div class="opakujici_most"></div>
			</div>
		</header>
		<main>
			<div class="opakujici_most"></div>
			<nav>
				<b><a href="/kontakty">{"KONTAKTY"}</a></b>
				<b><a href="/harmonogram">{"HARMONOGRAM"}</a></b>
				<b><a href="/o_akci">{"O AKCI"}</a></b>
			</nav>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

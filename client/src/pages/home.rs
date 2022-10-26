use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
	yew_hooks::use_title("Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<header class="uvod_header">
			<div class="title">
				<div class="first_line">
					<h1><a href="/">{"MOSTY"}</a></h1>
					<div class="opakujici_most"></div>
				</div>
				<p>{"SYMPOSION GYMNÁZIA JANA KEPLERA"}</p>
			</div>
			<div class={"date"}>
				<p>{"16. - 18. listopadu 2022"}</p>
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

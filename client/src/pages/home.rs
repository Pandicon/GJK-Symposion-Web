use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
	yew_hooks::use_title("Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<header class="uvod_header">
			<div class="title">
				<div class="first_line">
					<h1><a href="/"><span class="most">{"MOSTY"}</span></a></h1>
					<div class="opakujici_most"></div>
				</div>
				<p><span class="most">{"SYMPOSION GYMNÁZIA JANA KEPLERA"}</span></p>
			</div>
			<div class={"date"}>
				<p><span class="most">{"16. - 18. listopadu 2022"}</span></p>
				<div class="opakujici_most"></div>
			</div>
		</header>
		<main>
			<div class="opakujici_most"></div>
			<nav>
				<b><a href="/kontakty"><span class="most">{"KONTAKTY"}</span></a></b>
				<b><a href="/harmonogram"><span class="most">{"HARMONOGRAM"}</span></a></b>
				<b><a href="/o_akci"><span class="most">{"O AKCI"}</span></a></b>
			</nav>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

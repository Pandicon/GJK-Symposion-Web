use crate::{components::link_to::LinkTo, router::Route};

use yew::prelude::*;

/// The #NotFound page
/// This is the 404 not found page that will show up when you go to a link that doesn't exist
#[function_component(NotFound)]
pub fn not_found() -> Html {
	yew_hooks::use_title("404 | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<header class="uvod_header">
			<div class="title">
				<div class="first_line">
					<h1>
					<LinkTo path="/" route={Route::Home} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
						<span class="most">{"MOSTY"}</span>
					</LinkTo>
					</h1>
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
			<div class="mosty_jako" style="transform: rotate(-20deg)">
				<p class="most" style="transform: rotate(-10deg)">
					<h2 class="most" style="transform: rotate(25deg)">{"404"}</h2>
					<h3 class="most">{"Most do této destinace spadl"}</h3>
				</p>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

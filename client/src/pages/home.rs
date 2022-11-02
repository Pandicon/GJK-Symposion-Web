use crate::pages::{harmonogram::Harmonogram, kontakty::Kontakty, o_akci::OAkci};
use crate::{components::link_to::LinkTo, router::Route};

use yew::prelude::*;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	/// The config including the API base.
	pub config: crate::types::Config,
}

/// # The Home page
/// This page shows the page you see when you open the Symposion webpage
#[function_component(Home)]
pub fn home(props: &Props) -> Html {
	yew_hooks::use_title("Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
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
			<nav>
				<b>
				<LinkTo path="/kontakty" route={Route::Kontakty} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
					<span class="most">{"KONTAKTY"}</span>
				</LinkTo>
				</b>
				<b>
				<LinkTo path="/harmonogram" route={Route::HarmonogramAll} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
					<span class="most">{"HARMONOGRAM"}</span>
				</LinkTo>
				</b>
				<b>
				<LinkTo path="/o_akci" route={Route::OAkci} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
					<span class="most">{"O AKCI"}</span>
				</LinkTo>
				</b>
			</nav>
			<div class="mosty_jako">
				<p class="most">
					{"mosty jako podání ruky"}
					<br />
					{"mosty jako spojující prvek"}
					<br />
					{"mosty jako přechod, změna a vývoj"}
					<br />
					{"mosty jako způsob překonání překážek"}
					<br />
					{"mosty jako možnost spojení nespojitelného"}
				</p>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		if props.config.one_page {
			<OAkci />
			<Harmonogram config={props.config.clone()} />
			<Kontakty />
		}
		<footer>
		</footer>
		</>
	}
}

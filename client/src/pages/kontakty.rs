use crate::{components::link_to::LinkTo, router::Route};
use yew::prelude::*;

/// # The Kontakty page
/// This page displays the contacts relevant for the Symposion
#[function_component(Kontakty)]
pub fn kontakty() -> Html {
	yew_hooks::use_title("Kontakty | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
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
				<h2><span class="most">{"Kontakty"}</span></h2>
			</div>
		</header>
		<main>
			<div class="opakujici_most"></div>
			<div class="icons">
				<a href="https://fb.me/SymposionGjk"><img src="../../images/FACEBOOK.png" width="80px" alt="Facebook logo" /></a>
				<a href="https://instagram.com/symposion_gjk"><img src="../../images/INSTAGRAM.png" width="80px" alt="Instagram logo" /></a>
				<a href="mailto:symposion@gjk.cz"><img src="../../images/MAIL.png" width="80px" alt="Email logo" /></a>
			</div>
			<div class="address">
				<iframe id="gmaps" src="https://frame.mapy.cz/s/gudusubala" aria-hidden="false" tabindex="0" frameborder="0" title="Gymnázium Jana Keplera na mapě"></iframe>
				<div class="school_wrapper">
					<div class="school most">{"Gymnázium Jana Keplera"}</div>
					<img src="../../images/most_icon.png" width="90px" alt="Obrázek mostu" />
					<div class="school_address most">{"Parléřova 2, 169 00, Praha 6"}</div>
				</div>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

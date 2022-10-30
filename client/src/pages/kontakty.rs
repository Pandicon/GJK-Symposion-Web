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
				<img src="../../images/FACEBOOK.png" />
				<img src="../../images/INSTAGRAM.png" />
				<img src="../../images/MAIL.png" />
			</div>
			<div class="address">
				<iframe id="gmaps" src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2559.91428932703!2d14.385109315786583!3d50.08789157942699!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x470b951a87784a87%3A0x7cc9fd475f7b8166!2sGymn%C3%A1zium%20Jana%20Keplera!5e0!3m2!1scs!2scz!4v1609597010791!5m2!1scs!2scz" aria-hidden="false" tabindex="0" frameborder="0"></iframe>
				<div class="school most">{"Gymnázium Jana Keplera"}</div>
				<img src="../../images/most_icon.png" />
				<div class="school_address most">{"Parléřova 2, 169 00, Praha 6"}</div>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

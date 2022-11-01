use crate::{components::link_to::LinkTo, router::Route};
use yew::prelude::*;

/// # The OAkci page
/// This page will show information about the Symposion event
#[function_component(OAkci)]
pub fn o_akci() -> Html {
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
			<div class="about_wrapper">
				<h1 class="most_bez_s">
					{"Lorem ipsum dolor sit amet, consectetuer adipiscing elit."}
				</h1>
				<p class="most_bez_s">
					{"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas lorem. Fusce dui leo, imperdiet in, aliquam sit amet, feugiat eu, orci. Praesent vitae arcu tempor neque lacinia pretium. Phasellus enim erat, vestibulum vel, aliquam a, posuere eu, velit. Aenean fermentum risus id tortor. Proin pede metus, vulputate nec, fermentum fringilla, vehicula vitae, justo. Donec quis nibh at felis congue commodo. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla non lectus sed nisl molestie malesuada. Mauris suscipit, ligula sit amet pharetra semper, nibh ante cursus purus, vel sagittis velit mauris vel metus. Nullam faucibus mi quis velit. "}
				</p>
				<p class="most_bez_s">
					{"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas lorem. Fusce dui leo, imperdiet in, aliquam sit amet, feugiat eu, orci. Praesent vitae arcu tempor neque lacinia pretium. Phasellus enim erat, vestibulum vel, aliquam a, posuere eu, velit. Aenean fermentum risus id tortor. Proin pede metus, vulputate nec, fermentum fringilla, vehicula vitae, justo. Donec quis nibh at felis congue commodo. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla non lectus sed nisl molestie malesuada. Mauris suscipit, ligula sit amet pharetra semper, nibh ante cursus purus, vel sagittis velit mauris vel metus. Nullam faucibus mi quis velit. "}
				</p>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

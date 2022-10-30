use yew::prelude::*;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	/// The error to display
	pub error: String,
}

/// # The Chyba page
/// This page shows an error message in case something goes wrong and is caugth on the frontend
#[function_component(Chyba)]
pub fn chyba(props: &Props) -> Html {
	yew_hooks::use_title("Nastala chyba | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<main>
		<h1>{"Při načítání stránky nastala chyba, která je uvedená níže. Při komunikaci s vývojářským týmem jim prosím tuto chybu nahlašte:"}</h1>
		<div class={"error"}>{&props.error}</div>
		</main>
		<footer>
		</footer>
		</>
	}
}

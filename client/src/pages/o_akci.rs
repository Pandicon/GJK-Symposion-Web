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
					{"25. ročník Symposionu Gymnázia Jana Keplera je tu!"}
				</h1>
				<p class="most_bez_s">
					{"Zveme všechny příznivce vědění a poznání k podívané, poslechu a počtení, které vyzývá k hledání paralel a provokuje diskuzi. Toužíte-li po zasazení do kontextu a po propojení známého s neznámým, vyslechněte slova, která vám letos Symposion prostřednictvím našich skvělých hostů nabídne. Jistě již netrpělivě čekají, až budou moci své myšlenky a podněty přenést i na Vás, posluchače. Přednášku přislíbili přednášející všech možných zaměření a zájmů, teoretici i praktici."}
				</p>
				<p class="most_bez_s">
					{"Symposion se tradičně koná v listopadu při oslavách Dne studentstva přímo u nás na gymnáziu. Vítáni jsou všichni a vstup je volný. Během tří dnů probíhají bloky různorodých přednášek doprovázených například koncertem studentských kapel, výstavami, promítáním filmů či divadlem. Ve škole, ze které se na tyto krásné dny stává místo poznání, potěšení a inspirativních setkání, nechybí ani kavárna nebo dětský koutek."}
				</p>
			</div>
			<div class="opakujici_most_naopak"></div>
		</main>
		<footer>
		</footer>
		</>
	}
}

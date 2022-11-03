use yew::prelude::*;

/// # The Footer component
/// This component is the footer of all of the main pages and contains the website credits
#[function_component(Footer)]
pub fn footer() -> Html {
	html! {
		<footer>
			<h6 style="padding-top: 5px">
				{"Stránku vytvořili Martin Kudrna, Filip Majer, Vojtěch Černý a Tomáš Pryl"}<br />
				{"Design stránky vytvořila Julie Pförtnerová"}<br />
				{"S ❤️ k 🌍 "}
				<a href="https://github.com/Pandicon/GJK-Symposion-Web">{"napsané"}</a>
				{" v Rustu "}
				<a href="https://www.rust-lang.org/">
				<img src="/images/ferris.png" height="13px" alt="" />
				</a>
				{" a "}
				<a href="https://en.cppreference.com/w/">
				<img src="/images/c++.png" height="13px" alt="C++" />
				</a>
				{", jedněch z "}
				<a href="https://haslab.github.io/SAFER/scp21.pdf">{"energeticky nejúspornějších"}</a>
				{" programovacích jazyků"}
			</h6>
		</footer>
	}
}

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <h1>{"Nazdárek!"}</h1>
        <h2>{"Zde je hlavní stránka :D"}</h2>
        </>
    }
}
use crate::utils;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let local_storage = utils::get_local_storage();
    let value = utils::get_local_storage_key(&local_storage, "test");
    if value.is_none() {
        utils::set_local_storage_key(&local_storage, "test", "test value");
    }
    html! {
        <>
        <h1>{"Nazdárek!"}</h1>
        <h2>{"Zde je hlavní stránka :D"}</h2>
        if let Some(val) = value {
            <p>{val}</p>
        }
        </>
    }
}
use crate::utils;

use gloo::console;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let local_storage = utils::get_local_storage();
    let value = utils::get_local_storage_key(&local_storage, "test");
    if value.is_none() {
        if utils::set_local_storage_key(&local_storage, "test", &format!("Current timestamp is {}", js_sys::Date::now())).is_err() {
            console::log!("Something went wrong when putting the value into the local storage");
        }
    } else {
        if utils::remove_local_storage_key(&local_storage, "test").is_err() {
            console::log!("Something went wrong when removing the value from the local storage");
        }
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
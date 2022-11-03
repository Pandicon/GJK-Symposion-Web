use yew::prelude::*;

use crate::utils;

/// # The ClearStorage page
/// This page tries to clear the local storage, allowing you to bypass the 5 minute schedule cache
#[function_component(ClearStorage)]
pub fn clear_storage() -> Html {
	yew_hooks::use_title("Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	let mut message = String::from("Místní úložiště bylo úspešně smazané");
	if let Some(local_storage) = utils::get_local_storage() {
		if let Err(err) = local_storage.clear() {
			message = format!("Nastala chyba při mazání místního úložiště: {:?}", err);
		}
	} else {
		message = String::from("Nepodařilo se získat instanci místního úložiště");
	}
	html! {
		<>
		<h1>{message}</h1>
		</>
	}
}

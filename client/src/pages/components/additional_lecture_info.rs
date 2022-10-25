use yew::prelude::*;

use crate::types::AdditionalCellInfo;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	pub enabled_state: UseStateHandle<bool>,
	pub data_state: UseStateHandle<AdditionalCellInfo>,
}

#[function_component(AdditionalLectureInfo)]
pub fn home(props: &Props) -> Html {
	yew_hooks::use_title("Nastala chyba | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());
	html! {
		<>
		<div class="overlay-body" style={
			if *props.enabled_state {
				"display: block;"
			} else {
				"display: none;"
			}
		}><div>{format!("{:?} {:?} {:?}", props.data_state.data, props.data_state.warning, props.data_state.error)}</div><br /><br /><div onclick={let cloned_additional_cell_info_enabled_state = props.enabled_state.clone(); Callback::from(move |_| {cloned_additional_cell_info_enabled_state.set(false)})}>{"Zpět"}</div></div>
		</>
	}
}

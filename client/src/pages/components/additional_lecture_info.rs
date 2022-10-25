use yew::prelude::*;

use crate::types::AdditionalCellInfo;

use chrono::TimeZone;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	pub enabled_state: UseStateHandle<bool>,
	pub data_state: UseStateHandle<AdditionalCellInfo>,
}

#[function_component(AdditionalLectureInfo)]
pub fn home(props: &Props) -> Html {
	let utc_date = chrono::Utc.timestamp(props.data_state.last_updated, 0);
	let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
	html! {
		<>
		<div class="overlay-body" style={
			if *props.enabled_state {
				"display: block;"
			} else {
				"display: none;"
			}
		}>
			if let Some(data) = &props.data_state.data {
				<div class="overlay-lecturer">{&data.title}</div>
				<br />
				<div class="overlay-lecturer">{&data.lecturer}</div>
				if let Some(annotation) = &data.annotation {
					<br />
					<div class="overlay-annotation">{annotation}</div>
				}
				if let Some(lecturer_info) = &data.lecturer_info {
					<br />
					<div class="overlay-lecturer-info">{lecturer_info}</div>
				}
				if data.for_younger {
					<br />
					<i>{"Vhodné i pro mladší diváky"}</i>
				}
			}
			if let Some(warning) = &props.data_state.warning {
				<br />
				<div class="overlay-error">{warning}</div>
			}
			if let Some(error) = &props.data_state.error {
				<br />
				<div class="overlay-error">{error}</div>
			}
			<p>{update_date_local.format("Data z %d.%m.%Y %H:%M:%S").to_string()}</p>
			<div class="overlay-back" onclick={
				let cloned_additional_cell_info_enabled_state = props.enabled_state.clone();
				let cloned_data_state = props.data_state.clone();
				Callback::from(move |_| {
					cloned_additional_cell_info_enabled_state.set(false);
					cloned_data_state.set(AdditionalCellInfo::default());
				})
			}>{"Zpět"}</div>
		</div>
		</>
	}
}
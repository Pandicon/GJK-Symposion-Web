use yew::prelude::*;

use crate::{router::Route, types::AdditionalCellInfo, utils};

use chrono::TimeZone;
use yew_router::history::History;

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	/// The enabled state for the additional data component
	pub enabled_state: UseStateHandle<bool>,
	/// The additional data state
	pub data_state: UseStateHandle<AdditionalCellInfo>,
	/// The day on which the lecture the details will be shown takes place
	pub day: String,
}

/// # The AdditionalLectureInfo component
/// This component shows additional information about a lecture in the form of a pop-up. It includes the time, room, annotation, and some information about the lecturer.
#[function_component(AdditionalLectureInfo)]
pub fn additional_lecture_info(props: &Props) -> Html {
	let utc_date = chrono::Utc.timestamp(props.data_state.last_updated, 0);
	let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
	let day = props.day.clone();
	html! {
		<>
		<div class="overlay-background" style={
			if *props.enabled_state {
				"display: block;"
			} else {
				"display: none;"
			}
		}></div>
		<div class="overlay-body" style={
			if *props.enabled_state {
				"display: block;"
			} else {
				"display: none;"
			}
		}>
			if let Some(data) = &props.data_state.data {
				if !data.lecturer.trim().is_empty() {
					<div class="overlay-lecturer">
						<span class="most">{&data.lecturer}</span>
						<div class="opakujici_most"></div>
					</div>
				}
				<div class="overlay-lecture">
					<span class="most">{&data.title}</span>
				</div>
				if let (Some(start_time), Some(end_time)) = (&data.start_time, &data.end_time) {
					<div class="overlay-time">{utils::raw_harmonogram_day_to_display_day(&data.day)}{", "}{start_time}{" - "}{end_time}</div>
				}
				<div class="overlay-rooms">{data.lecture_rooms.join(", ")}</div>
				if let Some(annotation) = &data.annotation {
					<div class="overlay-annotation">{annotation}</div>
				}
				if let Some(lecturer_info) = &data.lecturer_info {
					<div class="overlay-lecturer-info">{lecturer_info}</div>
				}
				if data.for_younger {
					<i>{"Vhodn?? i pro mlad???? div??ky."}</i>
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
			<p class="data_from">{update_date_local.format("Data z %d.%m.%Y %H:%M:%S").to_string()}</p>
			if let Some(history) = yew_router::hooks::use_history() {
				<div class="overlay-back" onclick={
					let cloned_additional_cell_info_enabled_state = props.enabled_state.clone();
					let cloned_data_state = props.data_state.clone();
					Callback::from(move |_| {
						history.push(if day == *"all" {
							Route::HarmonogramAll
						} else {
							Route::Harmonogram {day: day.clone()}
						});
						cloned_additional_cell_info_enabled_state.set(false);
						cloned_data_state.set(AdditionalCellInfo::default());
					})
				}>{"Zp??t"}</div>
			} else {
				<a class="overlay-back" style="text-decoration: none; color: inherit;" href={
					if day == *"all" {
						String::from("/harmonogram")
					} else {
						format!("/harmonogram/{}", day)
					}
				}>{"Zp??t"}</a>
			}
		</div>
		</>
	}
}

use crate::router::Route;
use crate::types::{
	AdditionalCellInfo, AdditionalCellInfoBase, AdditionalCellInfoCache, AdditionalCellInfoData, AdditionalCellInfoResponse, HarmonogramData, HarmonogramDayCache, HarmonogramDayData,
	HarmonogramDayResponse, HarmonogramField, HarmonogramState,
};
use crate::utils;

use crate::components::{additional_lecture_info::AdditionalLectureInfo, link_to::LinkTo};

use chrono::TimeZone;
use yew::prelude::*;
use yew_router::history::History;

const VALID_DAYS: [&str; 3] = ["streda", "ctvrtek", "patek"];
const CACHE_LIFETIME: i64 = 5 * 60; // 5 minutes represented in seconds

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	/// The day for which to display the schedule. If it is missing, all days are rendered.
	pub day: Option<String>,
	/// The config including the API base.
	pub config: crate::types::Config,
	/// The Id of the lecture for which the details should be shown, if any.
	pub details_id: Option<String>,
}

/// # The Harmonogram page
/// This page handles rendering of the schedule and additional information about the lectures.
#[function_component(Harmonogram)]
pub fn harmonogram(props: &Props) -> Html {
	yew_hooks::use_title("Harmonogram | Mosty - Symposion 2022 | Gymnázium Jana Keplera".to_string());

	let api_base = &props.config.api;

	let additional_cell_info_state: UseStateHandle<AdditionalCellInfo> = use_state(AdditionalCellInfo::default);
	let additional_cell_info_enabled_state = use_state(|| false);
	let current_timestamp_seconds = chrono::offset::Local::now().timestamp();
	let day_from_url = if let Some(day) = &props.day {
		let day_lowercase = day.to_ascii_lowercase();
		if !VALID_DAYS.contains(&day_lowercase.as_str()) {
			String::from("all")
		} else {
			day_lowercase
		}
	} else {
		String::from("all")
	};

	let mut details_id = None;
	if let Some(id) = &props.details_id {
		let mut split_id = id.split('-').collect::<Vec<&str>>();
		let mut day = None;
		if VALID_DAYS.contains(&split_id[0]) {
			day = Some(split_id.remove(0).to_owned());
		} else if day_from_url != *"all" {
			day = Some(day_from_url.clone());
		}
		let filtered = split_id.iter().filter_map(|el| el.parse::<usize>().ok()).collect::<Vec<usize>>();
		if filtered.len() > 1 {
			details_id = Some((day, format!("{}-{}", filtered[0], filtered[1])));
		}
	}

	let harmonogram_state: UseStateHandle<HarmonogramState> = use_state(HarmonogramState::default);
	if harmonogram_state.data.is_none() && harmonogram_state.error.is_none() {
		set_harmonogram_state(harmonogram_state.clone(), api_base, current_timestamp_seconds, &day_from_url);
	}

	let days = match harmonogram_state.data.clone() {
		Some(data) => data,
		None => vec![],
	};
	let times_all_days = days
		.iter()
		.map(|(_day, day_data)| {
			day_data
				.harmonogram
				.iter()
				.map(|row| match &row[0] {
					Some(field) => {
						let time_split = &field.title.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
						if time_split.len() < 2 {
							None
						} else {
							Some([time_split[0], time_split[1]])
						}
					}
					None => None,
				})
				.collect::<Vec<Option<[&str; 2]>>>()
		})
		.collect::<Vec<Vec<Option<[&str; 2]>>>>();
	let rooms_all_days = days
		.iter()
		.map(|(_day, day_data)| {
			if let Some(row) = day_data.harmonogram.first() {
				row.iter().map(|cell_option| cell_option.as_ref().map(|cell| cell.title.clone())).collect::<Vec<Option<String>>>()
			} else {
				vec![None]
			}
		})
		.collect::<Vec<Vec<Option<String>>>>();
	if !*additional_cell_info_enabled_state || additional_cell_info_state.last_updated == 0 {
		if let Some(details_id) = details_id {
			if let Some(day_index) = if let Some(day) = details_id.0.clone() { days.iter().position(|el| el.0 == day) } else { None } {
				if let Some(Some(cell)) = days[day_index].1.harmonogram.iter().flatten().find(|cell_option| {
					if let Some(cell) = cell_option {
						if let Some(cell_id) = &cell.id {
							cell_id == &details_id.1
						} else {
							false
						}
					} else {
						false
					}
				}) {
					if !*additional_cell_info_enabled_state {
						additional_cell_info_enabled_state.set(true);
					}
					let (cell_day, cell_id) = details_id;
					let cell_day = cell_day.unwrap();
					let times = &times_all_days[day_index];
					let rooms = &rooms_all_days[day_index];
					let [col_span, row_span] = get_cell_spans(cell);
					let row_id = cell_id.split('-').collect::<Vec<&str>>()[0].parse().unwrap();
					let [start_time, end_time] = get_cell_start_end_times(row_id, row_span as usize, times);
					let lecture_rooms = get_lecture_rooms(&cell_id, col_span as usize, rooms);
					let base_info = AdditionalCellInfoBase {
						lecturer: cell.lecturer.clone(),
						title: cell.title.clone(),
						for_younger: cell.for_younger,
						start_time: if row_id > 0 { Some(start_time) } else { None },
						end_time: if row_id > 0 { Some(end_time) } else { None },
						lecture_rooms,
					};
					if additional_cell_info_state.last_updated == 0 {
						set_additional_info_state(additional_cell_info_state.clone(), api_base, current_timestamp_seconds, cell_day, cell_id, base_info);
					}
				};
			};
		}
	}
	html! {
		<>
		<header class="harmonogram_header">
			<h1>
			<LinkTo path="/" route={Route::Home} history_style="cursor: pointer;">
				<span class="most">{"MOSTY"}</span>
			</LinkTo>
			</h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2>
					<LinkTo path="/harmonogram" route={Route::HarmonogramAll} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
						<span class="most">{"Harmonogram"}</span>
					</LinkTo>
				</h2>
			</div>
			if day_from_url != *"all" {
				<b class="day"><span class="most">{utils::raw_harmonogram_day_to_display_day(&day_from_url).to_uppercase()}</span></b>
			}
		</header>
		<main>
		<div class="opakujici_most"></div>
		{
			days.iter().enumerate().map(|(day_index, (day, day_data))| {
				let utc_date = chrono::Utc.timestamp(day_data.last_updated, 0);
				let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
				let times = &times_all_days[day_index];
				let rooms = &rooms_all_days[day_index];
				html!{
					<>
					if day_from_url == *"all" {
						<div class="harmonogram_day_title">
							<LinkTo path={format!("/harmonogram/{}", day)} route={Route::Harmonogram { day: day.clone() }} link_style="text-decoration: none; color: inherit;" history_style="cursor: pointer;">
								<p class="most">{utils::raw_harmonogram_day_to_display_day(day)}</p>
							</LinkTo>
							<div class="opakujici_most"></div>
						</div>
					}
					<table class="harmonogram_day">
					{
						day_data.harmonogram.iter().enumerate().map(|(row_id, row)| {
							if row.is_empty() {
								return html!{};
							}
							let start_time = if let Some(time) = times[row_id] {
								time[0]
							} else {
								"???"
							};
							let prev_end_time = if row_id > 1 {
								if let Some(time) = times[row_id - 1] {
									time[1]
								} else {
									"???"
								}
							} else {
								start_time
							};
							html!{
								<tr>
									<td>{
										html!{
											<>
											if row_id > 0 {
												if prev_end_time != start_time {
													<div class="mobile_hidden nazev_prednasky">{ prev_end_time }<br /></div>
												}
												<span class="nazev_prednasky">{ start_time }</span>
											}
											</>
										}
									}</td>
									{
										row.iter().skip(1).map(|cell_option| {
										if let Some(cell) = cell_option {
											let [col_span, row_span] = get_cell_spans(cell);
											let mut lecture_rooms = vec![];
											let (class_name, href, on_click) = if let Some(cell_id) = &cell.id {
												lecture_rooms = get_lecture_rooms(cell_id, col_span as usize, rooms);
												if let Some(history) = yew_router::hooks::use_history() {
													let cloned_cell_id = cell_id.clone();
													let cloned_day = day.clone();
													let cloned_url_day = day_from_url.clone();
													("clickable", None, Callback::from(move |_| {
														history.push(if cloned_url_day == *"all" {
															Route::HarmonogramAllDetails { id: format!("{}-{}", cloned_day.clone(), cloned_cell_id.clone()) }
														} else {
															Route::HarmonogramDetails { day: cloned_url_day.clone(), id: cloned_cell_id.clone() }
														});
													}))
												} else {
													("", Some(if day_from_url == *"all" {
														format!("/harmonogram/details/{}-{}", day.clone(), cell_id.clone())
													} else {
														format!("/harmonogram/{}/details/{}", day.clone(), cell_id.clone())
													}), Callback::from(|_| {}))
												}
											} else {
												("", None, Callback::from(|_| {}))
											};
											html!{
												<td class={class_name} colspan={format!("{col_span}")} rowspan={format!("{row_span}")} onclick={on_click}>
													<a href={href}>
														<b>{&cell.lecturer}</b>
														<span class="nazev_prednasky">{ &cell.title }</span>
														if cell.for_younger {
															<div class="for_younger">{"*"}</div>
														}
														if row_id > 0 {
															/*{"Start: "}{start_time}{" Konec: "}{end_time}*/
															if !lecture_rooms.is_empty() {
																<div class="lecture_room">
																	{lecture_rooms.join(", ")}
																</div>
															}
														}
													</a>
												</td>
											}
										} else {
											html!{
												<td />
											}
										}
									}).collect::<Html>()
								}
								</tr>
							}
						}).collect::<Html>()
					}
					if times.len() >= day_data.harmonogram.len() {
						if let Some(end_time) = times[day_data.harmonogram.len() - 1] {
							<tr><td><span class="nazev_prednasky">{end_time[1]}</span></td></tr>
						}
					}
					</table>
					<p class="data_from">{update_date_local.format("Data z %d.%m.%Y %H:%M:%S").to_string()}</p>
					</>
				}
			}).collect::<Html>()
		}
		<AdditionalLectureInfo enabled_state={additional_cell_info_enabled_state.clone()} data_state={additional_cell_info_state.clone()} day={day_from_url}/>
		<div class="opakujici_most_naopak"></div>
		</main>
		<footer></footer>
		</>
	}
}

fn set_additional_info_state(state: UseStateHandle<AdditionalCellInfo>, api_base: &str, current_timestamp_seconds: i64, day: String, id: String, base_info: AdditionalCellInfoBase) {
	let AdditionalCellInfoBase {
		lecturer,
		title,
		for_younger,
		start_time,
		end_time,
		lecture_rooms,
	} = base_info;
	let mut data_to_set = AdditionalCellInfo::new(
		Some(AdditionalCellInfoData {
			lecturer: lecturer.clone(),
			title: title.clone(),
			start_time: start_time.clone(),
			end_time: end_time.clone(),
			for_younger,
			annotation: None,
			lecturer_info: None,
			lecture_rooms: lecture_rooms.clone(),
			day: day.clone(),
		}),
		None,
		None,
		current_timestamp_seconds,
	);
	let utc_date = chrono::Utc.timestamp(current_timestamp_seconds, 0);
	let current_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
	let current_date_formatted = current_date_local.format("%d.%m.%Y %H:%M:%S").to_string();
	if let Some(cache) = get_harmonogram_additional_data_cache(&day, &id) {
		data_to_set = AdditionalCellInfo {
			data: Some(cache.data),
			warning: None,
			error: None,
			last_updated: cache.last_updated,
		};
		if cache.timestamp >= current_timestamp_seconds - CACHE_LIFETIME {
			state.set(data_to_set);
			return;
		}
	}
	let api_base = api_base.to_owned();
	wasm_bindgen_futures::spawn_local(async move {
		gloo::console::debug!(format!("Fetching additional cell data for cell {id} and day {day} from the API"));
		match gloo::net::http::Request::get(&format!("{}/anotace/{}/{}", api_base, day, id)).send().await {
			Ok(response) => {
				if !response.ok() {
					gloo::console::error!(format!("The response was not 200 OK: {:?}", response.status_text()));
					data_to_set.error = Some(format!(
						"(Chyba z {}) Server odpověděl se statusem {}: {}",
						current_date_formatted,
						response.status(),
						response.status_text()
					));
				} else {
					match response.text().await {
						Ok(text) => match serde_json::from_str::<AdditionalCellInfoResponse>(&text) {
							Ok(data) => match data.data {
								Some(data) => {
									let cell_info_data = AdditionalCellInfoData {
										lecturer,
										title,
										for_younger,
										start_time,
										end_time,
										annotation: data.info.annotation,
										lecturer_info: data.info.lecturer_info,
										lecture_rooms,
										day: day.clone(),
									};
									set_harmonogram_additional_data_cache(&day, &id, current_timestamp_seconds, cell_info_data.clone(), data.last_updated);
									let state_data = AdditionalCellInfo::new(Some(cell_info_data), None, None, data.last_updated);
									state.set(state_data);
									return;
								}
								_ => {
									if let Some(error) = data.error {
										data_to_set.error = Some(format!("(Chyba z {}) {:?}", current_date_formatted, error))
									} else if let Some(data_to_cache) = data_to_set.data.clone() {
										set_harmonogram_additional_data_cache(&day, &id, current_timestamp_seconds, data_to_cache, current_timestamp_seconds);
									}
								}
							},
							Err(error) => {
								gloo::console::error!(format!("Failed to deserialize the response: {:?}", error));
								data_to_set.error = Some(format!("(Chyba z {}) Nepodařilo se převést odpověď serveru do správného formátu: {:?}", current_date_formatted, error));
							}
						},
						Err(error) => {
							gloo::console::error!(format!("Couldn't get the response text: {:?}", error));
							data_to_set.error = Some(format!("(Chyba z {}) Nepodařilo se získat text odpovědi serveru: {:?}", current_date_formatted, error));
						}
					}
				}
			}
			Err(error) => {
				gloo::console::error!(format!("Something went wrong when fetching the API: {:?}", error));
				data_to_set.error = Some(format!("(Chyba z {}) Nepodařilo se získat odpověď serveru: {:?}", current_date_formatted, error));
			}
		}
		state.set(data_to_set);
	})
}

fn set_harmonogram_state(state: UseStateHandle<HarmonogramState>, api_base: &str, current_timestamp_seconds: i64, day: &str) {
	let cache = get_harmonogram_cache(day);
	let api_base = api_base.to_owned();
	wasm_bindgen_futures::spawn_local(async move {
		let mut days = vec![];
		for (i, day_cache_all) in cache.iter().enumerate() {
			let day = &day_cache_all.day;
			let day_cache = day_cache_all.cache.as_ref();
			days.push((day.to_owned(), None));
			if let Some(day_cache_res) = day_cache {
				let timestamp = day_cache_res.timestamp;
				days[i] = (day.to_owned(), Some(day_cache_res.to_owned().data));
				if timestamp >= current_timestamp_seconds - CACHE_LIFETIME {
					continue;
				}
			}
			gloo::console::debug!(format!("Fetching the schedule from the API for day {}", &day));
			match gloo::net::http::Request::get(&format!("{}/{}", api_base, day)).send().await {
				Ok(response) => {
					if !response.ok() {
						gloo::console::error!(format!("The reponse was not 200 OK: {:?}", response.status_text()));
					} else {
						match response.text().await {
							Ok(text) => match serde_json::from_str::<HarmonogramDayResponse>(&text) {
								Ok(data) => {
									if let Some(data) = data.data {
										set_harmonogram_cache(day, current_timestamp_seconds, data.clone());
										days[i] = (day.to_owned(), Some(data));
									}
								}
								Err(error) => {
									gloo::console::error!(format!("Failed to deserialize the response: {:?}", error));
								}
							},
							Err(error) => {
								gloo::console::error!(format!("Couldn't get the response text: {:?}", error));
							}
						}
					}
				}
				Err(error) => {
					gloo::console::error!(format!("Something went wrong when fetching the API: {:?}", error));
				}
			}
		}
		let data_to_set = days.iter().filter(|day| day.1.is_some()).map(|day| (day.0.clone(), day.1.clone().unwrap())).collect();
		state.set(HarmonogramState::new(Some(data_to_set), None));
	});
}

fn get_harmonogram_cache(day: &str) -> Vec<HarmonogramDayData> {
	let mut res = vec![];
	let days = if day == "all" { VALID_DAYS.to_vec() } else { vec![day] };
	let local_storage = utils::get_local_storage();
	for day in days {
		let cache = utils::get_local_storage_key(&local_storage, &format!("harmonogram-{day}"));
		if let Some(data) = cache {
			if let Ok(parsed_data) = serde_json::from_str(&data) {
				res.push(HarmonogramDayData {
					day: day.to_owned(),
					cache: Some(parsed_data),
				});
			} else {
				res.push(HarmonogramDayData { day: day.to_owned(), cache: None });
			}
		} else {
			res.push(HarmonogramDayData { day: day.to_owned(), cache: None });
		}
	}
	res
}

fn set_harmonogram_cache(day: &str, timestamp: i64, data: HarmonogramData) {
	let cache = HarmonogramDayCache { data, timestamp };
	let local_storage = utils::get_local_storage();
	match serde_json::to_string(&cache) {
		Ok(data) => {
			if let Err(error) = utils::set_local_storage_key(&local_storage, &format!("harmonogram-{day}"), &data) {
				gloo::console::error!(format!("Failed to save cache to local storage: {}", error));
			}
		}
		Err(error) => gloo::console::error!(format!("Failed to parse cache to string: {}", error)),
	};
}

fn get_harmonogram_additional_data_cache(day: &str, id: &str) -> Option<AdditionalCellInfoCache> {
	let local_storage = utils::get_local_storage();
	let cache = utils::get_local_storage_key(&local_storage, &format!("anotace-{day}--{id}"));
	if let Some(data) = cache {
		if let Ok(parsed_data) = serde_json::from_str(&data) {
			return Some(parsed_data);
		}
	}
	None
}

fn set_harmonogram_additional_data_cache(day: &str, id: &str, timestamp: i64, data: AdditionalCellInfoData, last_updated: i64) {
	let cache = AdditionalCellInfoCache { data, timestamp, last_updated };
	let local_storage = utils::get_local_storage();
	match serde_json::to_string(&cache) {
		Ok(data) => {
			if let Err(error) = utils::set_local_storage_key(&local_storage, &format!("anotace-{day}--{id}"), &data) {
				gloo::console::error!(format!("Failed to save cache to local storage: {}", error));
			}
		}
		Err(error) => gloo::console::error!(format!("Failed to parse cache to string: {}", error)),
	};
}

fn get_lecture_rooms(cell_id: &str, col_span: usize, rooms: &[Option<String>]) -> Vec<String> {
	let mut lecture_rooms = vec![];
	let id_split = cell_id.split('-').collect::<Vec<&str>>();
	if let Ok(column_id) = id_split[1].trim().parse::<usize>() {
		for i in 0..col_span {
			let col = column_id + i;
			if col >= rooms.len() {
				break;
			}
			if let Some(room) = &rooms[col] {
				lecture_rooms.push(room.clone());
			}
		}
	};
	lecture_rooms
}

fn get_cell_spans(cell: &HarmonogramField) -> [u8; 2] {
	let col_span = if let Some(span) = cell.col_span { span } else { 1 };
	let row_span = if let Some(span) = cell.row_span { span } else { 1 };
	[col_span, row_span]
}

fn get_cell_start_end_times(row_id: usize, row_span: usize, times: &[Option<[&str; 2]>]) -> [String; 2] {
	let start_time = if row_id < times.len() {
		if let Some(time) = times[row_id] {
			time[0]
		} else {
			"???"
		}
	} else {
		"???"
	};
	let end_index = row_id + row_span - 1;
	let end_time = if end_index < times.len() {
		if let Some(time) = times[end_index] {
			time[1]
		} else {
			"???"
		}
	} else {
		"???"
	};
	[start_time.to_string(), end_time.to_string()]
}

use crate::types::{
	AdditionalCellInfo, AdditionalCellInfoBase, AdditionalCellInfoCache, AdditionalCellInfoData, AdditionalCellInfoResponse, HarmonogramData, HarmonogramDayCache, HarmonogramDayData,
	HarmonogramDayResponse, HarmonogramState,
};
use crate::utils;

use super::components::additional_lecture_info::AdditionalLectureInfo;

use chrono::TimeZone;
use yew::prelude::*;

const VALID_DAYS: [&str; 3] = ["streda", "ctvrtek", "patek"];
const CACHE_LIFETIME: i64 = 5 * 60; // 5 minutes represented in seconds

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	pub day: Option<String>,
	pub config: crate::types::Config,
}

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

	let harmonogram_state: UseStateHandle<HarmonogramState> = use_state(HarmonogramState::default);
	if harmonogram_state.data.is_none() && harmonogram_state.error.is_none() {
		set_harmonogram_state(harmonogram_state.clone(), api_base, current_timestamp_seconds, &day_from_url);
	}

	let days = match harmonogram_state.data.clone() {
		Some(data) => data,
		None => vec![],
	};
	html! {
		<>
		<header class="harmonogram_header">
			<h1><a href="/"><span class="most">{"MOSTY"}</span></a></h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2>
					<a href="/harmonogram" style="text-decoration: none; color: inherit;">
						<span class="most">{"Harmonogram"}</span>
					</a>
				</h2>
			</div>
			if day_from_url != *"all" {
				<b class="day"><span class="most">{utils::raw_harmonogram_day_to_display_day(&day_from_url).to_uppercase()}</span></b>
			}
		</header>
		<main>
		<div class="opakujici_most"></div>
		{
			days.iter().map(|(day, day_data)| {
				let utc_date = chrono::Utc.timestamp(day_data.last_updated, 0);
				let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
				let times = day_data.harmonogram.iter().map(|row| {
					match &row[0] {
						Some(field) => {
							let time_split = &field.title.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
							if time_split.len() < 2 {
								None
							} else {
								Some([time_split[0], time_split[1]])
							}
						},
						None => None
					}
				}).collect::<Vec<Option<[&str; 2]>>>();
				let rooms = if let Some(row) = day_data.harmonogram.first() {
					row.iter().map(|cell_option| {
						if let Some(cell) = cell_option {
							Some(cell.title.clone())
						} else {
							None
						}
					}).collect::<Vec<Option<String>>>()
				} else {
					vec![None]
				};
				html!{
					<>
					if day_from_url == *"all" {
						<div class="harmonogram_day_title">
							<a href={format!("/harmonogram/{}", day)} style="text-decoration: none; color: inherit;">
								<p class="most">{utils::raw_harmonogram_day_to_display_day(day)}</p>
							</a>
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
											let col_span = if let Some(span) = cell.col_span {
												span
											} else {
												1
											};
											let row_span = if let Some(span) = cell.row_span {
												span
											} else {
												1
											};
											let start_time = if row_id < times.len() {
												if let Some(time) = times[row_id] {
													time[0]
												} else {
													"???"
												}
											} else {
												"???"
											};
											let end_index = row_id + (row_span as usize) - 1;
											let end_time = if end_index < times.len() {
												if let Some(time) = times[end_index] {
													time[1]
												} else {
													"???"
												}
											} else {
												"???"
											};
											let cell_day = day.clone();
											let mut lecture_rooms = vec![];
											let (class_name, on_click) = if let Some(cell_id) = &cell.id {
												let id_split = cell_id.split('-').collect::<Vec<&str>>();
												if let Ok(column_id) = id_split[1].trim().parse::<usize>() {
													for i in 0..(col_span as usize) {
														let col = column_id + i;
														if col >= rooms.len() {
															break;
														}
														if let Some(room) = &rooms[col] {
															lecture_rooms.push(room.clone());
														}
													}
												};
												let cloned_cell_id = cell_id.clone();
												let cloned_additional_info_state = additional_cell_info_state.clone();
												let cloned_additional_cell_info_enabled_state = additional_cell_info_enabled_state.clone();
												let cloned_api_base = api_base.clone();
												let cloned_cell = cell.clone();
												let cloned_start_time = start_time.to_string();
												let cloned_end_time = end_time.to_string();
												let cloned_rooms = lecture_rooms.clone();
												("clickable", Callback::from(move |_| {
													cloned_additional_cell_info_enabled_state.set(true);
													let base_info = AdditionalCellInfoBase {
														lecturer: cloned_cell.lecturer.clone(),
														title: cloned_cell.title.clone(),
														for_younger: cloned_cell.for_younger,
														start_time: if row_id > 0 { Some(cloned_start_time.clone()) } else { None },
														end_time: if row_id > 0 { Some(cloned_end_time.clone()) } else { None },
														lecture_rooms: cloned_rooms.clone()
													};
													set_additional_info_state(cloned_additional_info_state.clone(), &cloned_api_base, current_timestamp_seconds, cell_day.clone(), cloned_cell_id.clone(), base_info);
												}))
											} else {
												("", Callback::from(|_| {}))
											};
											html!{
												<td class={class_name} colspan={format!("{col_span}")} rowspan={format!("{row_span}")} onclick={on_click}>
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
		<AdditionalLectureInfo enabled_state={additional_cell_info_enabled_state.clone()} data_state={additional_cell_info_state.clone()}/>
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

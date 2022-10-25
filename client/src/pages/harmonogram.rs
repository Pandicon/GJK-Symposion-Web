use crate::types::{AdditionalCellInfo, HarmonogramData, HarmonogramDayCache, HarmonogramDayData, HarmonogramDayResponse, HarmonogramState};
use crate::utils;

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
	gloo::console::log!("API base: ", api_base);

	let additional_cell_info_state: UseStateHandle<AdditionalCellInfo> = use_state(|| AdditionalCellInfo::default());
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

	let harmonogram_state: UseStateHandle<HarmonogramState> = use_state(|| HarmonogramState::default());
	if harmonogram_state.data.is_none() && harmonogram_state.error.is_none() {
		set_harmonogram_state(harmonogram_state.clone(), api_base, current_timestamp_seconds, &day_from_url);
	}

	let days = match harmonogram_state.data.clone() {
		Some(data) => data,
		None => vec![],
	};
	html! {
		<>
		<header>
			<h1><a href="/">{"MO$TY"}</a></h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2>{"Harmonogram"}</h2>
			</div>
			if day_from_url != *"all" {
				<b class="day">{utils::raw_harmonogram_day_to_display_day(&day_from_url).to_uppercase()}</b>
			}
		</header>
		<main>
		<div class="opakujici_most"></div>
		{
			days.iter().map(|(day, day_data)| {
				let utc_date = chrono::Utc.timestamp(day_data.last_updated, 0);
				let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
				html!{
					<>
					if day_from_url == *"all" {
						{utils::raw_harmonogram_day_to_display_day(day)}
					}
					<table style="width:100%">
					{
						day_data.harmonogram.iter().map(|row| {
							html!{
								<tr>
								{
									row.iter().map(|cell_option| {
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
											let cell_day = day.clone();
											let (class_name, on_click) = if let Some(cell_id) = cell.id.clone() {
												let cloned_additional_info_state = additional_cell_info_state.clone();
												("clickable", Callback::from(move |_| {
													let cloned_additional_info_state = cloned_additional_info_state.clone();
													gloo::console::log!(format!("Hello! Cell id: {}, Day: {}", cell_id, cell_day));
													// TODO: Make this an API call to our API
													wasm_bindgen_futures::spawn_local(async move {
														match gloo::net::http::Request::get("https://randomuser.me/api/").send().await {
															Ok(response) => {
																if !response.ok() {
																	gloo::console::error!(format!("The reponse was not 200 OK: {:?}", response.status_text()));
																	cloned_additional_info_state.set(AdditionalCellInfo::new(None, None, Some(format!("Nastala chyba, server odpověděl se statusem {}: {}", response.status(), response.status_text()))));
																} else {
																	match response.text().await {
																		Ok(text) => {
																			gloo::console::log!(format!("{:?}", text));
																			cloned_additional_info_state.set(AdditionalCellInfo::new(Some(text), None, None));
																		}
																		Err(error) => {
																			gloo::console::error!(format!("Couldn't get the response text: {:?}", error));
																			cloned_additional_info_state.set(AdditionalCellInfo::new(None, None, Some(format!("Nastala chyba, nepodařilo se získat text odpovědi serveru: {:?}", error))));
																		}
																	}
																}
															}
															Err(error) => {
																gloo::console::error!(format!("Something went wrong when fetching the API: {:?}", error));
																cloned_additional_info_state.set(AdditionalCellInfo::new(None, None, Some(format!("Nastala chyba, nepodařilo se získat odpověď serveru: {:?}", error))));
															}
														}
													})
												}))
											} else {
												("", Callback::from(|_| {}))
											};
											html!{
												<td class={class_name} colspan={format!("{col_span}")} rowspan={format!("{row_span}")} onclick={on_click}>
													<b>{&cell.lecturer}</b><br />{&cell.title}
													if cell.for_younger {
														<br /><i>{"Vhodné i pro mladší diváky"}</i>
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
					</table>
					<p>{update_date_local.format("Data z %d.%m.%Y %H:%M:%S").to_string()}</p>
					</>
				}
			}).collect::<Html>()
		}
		<div class="overlay-body"><div>{format!("{:?} {:?} {:?}", additional_cell_info_state.data, additional_cell_info_state.warning, additional_cell_info_state.error)}</div><br /><br /></div>
		<div class="opakujici_most_naopak"></div>
		</main>
		<footer></footer>
		</>
	}
}

fn set_harmonogram_state(state: UseStateHandle<HarmonogramState>, api_base: &str, current_timestamp_seconds: i64, day: &str) {
	let cache = get_harmonogram_cache(day);
	let mut fetch_data = false;
	for day_cache_all in &cache {
		let day_cache = day_cache_all.cache.as_ref();
		if day_cache.is_none() || day_cache.as_ref().unwrap().timestamp < current_timestamp_seconds - CACHE_LIFETIME {
			fetch_data = true;
			break;
		}
	}
	if !fetch_data {
		let mut days = vec![];
		for day_cache_all in &cache {
			let day = &day_cache_all.day;
			let day_cache = day_cache_all.cache.as_ref();
			if day_cache.is_some() && day_cache.as_ref().unwrap().timestamp >= current_timestamp_seconds - CACHE_LIFETIME {
				days.push((day.to_owned(), day_cache.unwrap().to_owned().data));
			}
		}
		state.set(HarmonogramState::new(Some(days), None));
	} else {
		gloo::console::debug!("Fetching the schedule from API");
		let api_base = api_base.to_owned();
		wasm_bindgen_futures::spawn_local(async move {
			let mut days = vec![];
			for day_cache_all in &cache {
				let day = &day_cache_all.day;
				match gloo::net::http::Request::get(&format!("{}/{}", api_base, day)).send().await {
					Ok(response) => {
						if !response.ok() {
							gloo::console::error!(format!("The reponse was not 200 OK: {:?}", response.status_text()));
						} else {
							match response.text().await {
								Ok(text) => {
									gloo::console::log!(format!("{:?}", text));
									match serde_json::from_str::<HarmonogramDayResponse>(&text) {
										Ok(data) => match data.data {
											Some(data) => {
												set_harmonogram_cache(&day, current_timestamp_seconds, data.clone());
												days.push((day.to_owned(), data))
											}
											_ => {}
										},
										Err(error) => {
											gloo::console::error!(format!("Failed to deserialize the response: {:?}", error));
										}
									}
								}
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
			state.set(HarmonogramState::new(Some(days), None));
		});
	}
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

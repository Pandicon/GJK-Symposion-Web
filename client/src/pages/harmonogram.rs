use crate::utils;

use chrono::TimeZone;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

const VALID_DAYS: [&str; 3] = ["streda", "ctvrtek", "patek"];
const CACHE_LIFETIME: i64 = 5 * 60; // 5 minutes represented in seconds

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
	pub day: Option<String>,
}

#[function_component(Harmonogram)]
pub fn harmonogram(props: &Props) -> Html {
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
	let mut days = vec![];
	let cache = get_harmonogram_cache(&day_from_url);
	for day_cache_all in &cache {
		let day = &day_cache_all.day;
		let day_cache = day_cache_all.cache.as_ref();
		if day_cache.is_some() && day_cache.as_ref().unwrap().timestamp >= current_timestamp_seconds - CACHE_LIFETIME {
			days.push((day.to_owned(), day_cache.unwrap().to_owned().data));
		} else {
			gloo::console::debug!("Fetching the API");
			let response_raw = r#"
            {"data":{"harmonogram":[[null,{"lecturer":"","title":"LCH","for_younger":false,"id":null},{"lecturer":"","title":"Sklep GJK","for_younger":false,"id":"0-2"},{"lecturer":"","title":"Strecha GJK","for_younger":false,"id":"0-3"}],[{"lecturer":"","title":"9:05 - 11:05","for_younger":false,"id":null},{"lecturer":"<script>alert('cheche!');</script>","title":"<script>alert('hehe!');</script>","for_younger":false,"id":null,"row_span":2},null,null],[{"lecturer":"","title":"11:05 - 11:55","for_younger":false,"id":null},{"lecturer":"prednasejici #1","title":"vysoce narocne tema tykajici se mostu","for_younger":false,"id":"2-2"},null],[{"lecturer":"","title":"12:01 - 13:02","for_younger":false,"id":null},{"lecturer":"prednasejici #3","title":"odpalování mostů","for_younger":false,"id":"3-1"},{"lecturer":"pan prednasejici #1","title":"symposion web stranky jako most mezi organizatory a ucastniky","for_younger":true,"id":"3-2","row_span":2},{"lecturer":"pani prednasejici #1","title":"rezonance mostu ve vetru","for_younger":true,"id":"3-3","row_span":2}],[{"lecturer":"","title":"13:02 - 23:42","for_younger":false,"id":null},null],[{"lecturer":"","title":"23:42 - 23:57","for_younger":false,"id":null},{"lecturer":"","title":"VEČEŘE","for_younger":true,"id":"5-1","col_span":3}],[{"lecturer":"","title":"23:59 - 24:00","for_younger":false,"id":null},null,null,{"lecturer":"prednasejici #2","title":"pozorování hvězd na téma most","for_younger":false,"id":"6-3"}]],"last_updated":1666606780},"error":null}
            "#; // TODO: Make this an actual API call once the API is set up
			let response = serde_json::from_str::<HarmonogramDayResponse>(response_raw);
			if let Ok(schedule) = response {
				if let Some(data) = schedule.data {
					set_harmonogram_cache(day, current_timestamp_seconds, data.clone());
					days.push((day.to_owned(), data));
				} else if let Some(error) = schedule.error {
					gloo::console::error!("Received an error: ", format!("{}", error));
				} else {
					gloo::console::error!("Didn't receive neither data nor an error");
				}
			} else {
				gloo::console::error!("Failed to parse my data: ", format!("{}", response.err().unwrap()));
			}
		}
	}
	html! {
		<>
		<header>
			<h1><a href="/">{"MOSTY"}</a></h1>
			<div class="hlavicka_most_nad">
				<div class="opakujici_most"></div>
				<h2>{"harmonogram"}</h2>
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
												("clickable", Callback::from(move |_| {
													gloo::console::log!(format!("Hello! Cell id: {}, Day: {}", cell_id, cell_day));
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
		<div class="opakujici_most_naopak"></div>
		</main>
		<footer></footer>
		</>
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct HarmonogramDayData {
	day: String,
	cache: Option<HarmonogramDayCache>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HarmonogramDayResponse {
	data: Option<HarmonogramData>,
	error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HarmonogramDayCache {
	data: HarmonogramData,
	timestamp: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HarmonogramData {
	harmonogram: Vec<Vec<Option<HarmonogramField>>>,
	last_updated: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HarmonogramField {
	col_span: Option<u8>,
	row_span: Option<u8>,
	for_younger: bool,
	id: Option<String>,
	lecturer: String,
	title: String,
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

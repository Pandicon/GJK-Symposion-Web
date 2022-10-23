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
	let day = if let Some(day) = &props.day {
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
	let cache = get_harmonogram_cache(&day);
	for day_cache_all in &cache {
		let day = &day_cache_all.day;
		let day_cache = day_cache_all.cache.as_ref();
		if day_cache.is_some() && day_cache.as_ref().unwrap().timestamp >= current_timestamp_seconds - CACHE_LIFETIME {
			days.push((day, day_cache.unwrap().to_owned().data));
		} else {
			let response_raw = r#"{
                "data": {
                    "harmonogram": [
                        [
                            { "lecturer": "", "title": "" },
                            { "lecturer": "", "title": "USV" },
                            { "lecturer": "", "title": "P2.3" },
                            { "lecturer": "", "title": "Tělocvična" }
                        ],
                        [
                            { "lecturer": "", "title": "8:00 - 9:00" },
                            { "lecturer": "Person 1", "title": "Lecture 1" },
                            { "lecturer": "Person 2", "title": "Lecture 2" },
                            { "row_span": 2, "lecturer": "Person 3", "title": "Lecture 3" }
                        ],
                        [
                            { "lecturer": "", "title": "9:00-10:30" },
                            { "col_span": 2, "lecturer": "Person 4", "title": "Lecture 4" }
                        ]
                    ],
                    "last_updated": 1666531231
                },
                "error": null
            }
            "#; // TODO: Make this an actual API call once the API is set up
			let response = serde_json::from_str::<HarmonogramDayResponse>(response_raw);
			if let Ok(schedule) = response {
				if let Some(data) = schedule.data {
					days.push((day, data));
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
		<h1>{"Nazdárek!"}</h1>
		<h2>{"Zde je harmonogram :D"}</h2>
		{
			days.iter().map(|(day, day_data)| {
				let utc_date = chrono::Utc.timestamp(day_data.last_updated, 0);
				let update_date_local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(utc_date);
				html!{
					<>
					{utils::raw_harmonogram_day_to_display_day(day)}
					<table style="width:100%">
					{
						day_data.harmonogram.iter().map(|row| {
							html!{
								<tr>
								{
									row.iter().map(|cell| {
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
										html!{
											<td colspan={format!("{col_span}")} rowspan={format!("{row_span}")}>
												<b>{&cell.lecturer}</b><br />{&cell.title}
											</td>
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
	harmonogram: Vec<Vec<HarmonogramField>>,
	last_updated: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HarmonogramField {
	col_span: Option<u8>,
	row_span: Option<u8>,
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

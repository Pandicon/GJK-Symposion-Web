use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, yew::Properties)]
pub struct Config {
	pub api: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HarmonogramDayData {
	pub day: String,
	pub cache: Option<HarmonogramDayCache>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramDayResponse {
	pub data: Option<HarmonogramData>,
	pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramDayCache {
	pub data: HarmonogramData,
	pub timestamp: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramData {
	pub harmonogram: Vec<Vec<Option<HarmonogramField>>>,
	pub last_updated: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramField {
	pub col_span: Option<u8>,
	pub row_span: Option<u8>,
	pub for_younger: bool,
	pub id: Option<String>,
	pub lecturer: String,
	pub title: String,
}

#[derive(Default, Debug, PartialEq)]
pub struct AdditionalCellInfo {
	pub data: Option<AdditionalCellInfoData>,
	pub warning: Option<String>,
	pub error: Option<String>,
	pub last_updated: i64,
}

impl AdditionalCellInfo {
	pub fn new(data: Option<AdditionalCellInfoData>, warning: Option<String>, error: Option<String>, last_updated: i64) -> Self {
		Self { data, warning, error, last_updated }
	}
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AdditionalCellInfoData {
	pub lecturer: String,
	pub title: String,
	pub for_younger: bool,
	pub annotation: Option<String>,
	pub lecturer_info: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponse {
	pub data: Option<AdditionalCellInfoResponseData>,
	pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponseData {
	pub info: AdditionalCellInfoResponseInfo,
	pub last_updated: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponseInfo {
	pub annotation: Option<String>,
	pub lecturer_info: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HarmonogramState {
	pub data: Option<Vec<(String, HarmonogramData)>>,
	pub error: Option<String>,
}
impl HarmonogramState {
	pub fn new(data: Option<Vec<(String, HarmonogramData)>>, error: Option<String>) -> Self {
		Self { data, error }
	}
}

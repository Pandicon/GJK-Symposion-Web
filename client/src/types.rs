use serde_derive::{Deserialize, Serialize};

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

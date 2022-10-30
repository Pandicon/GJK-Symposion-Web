use serde_derive::{Deserialize, Serialize};

/// The app configuration format
#[derive(Debug, Deserialize, Serialize, PartialEq, yew::Properties)]
pub struct Config {
	/// The API base to use
	pub api: String,
}

/// The base additional lecture information data which will always be present and can be obtained from the schedule data itself (without calling the `/anotace` API endpoint)
pub struct AdditionalCellInfoBase {
	/// The lecturer's name
	pub lecturer: String,
	/// The lecture title
	pub title: String,
	/// Whether or not the lecture is suitable for younger audience
	pub for_younger: bool,
	/// The start time of the lecture (if any)
	pub start_time: Option<String>,
	/// The end time of the lecture (if any)
	pub end_time: Option<String>,
	/// The rooms the lecture will take place at
	pub lecture_rooms: Vec<String>,
}

/// The cache structure for the additional lecture info
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoCacheData {
	/// The cache Id
	pub id: String,
	/// The cache itself (if any)
	pub cache: Option<AdditionalCellInfoCache>,
}

/// The cache data structure for the additional lecture info.
/// This is what is saved in the browser.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoCache {
	/// The cache data
	pub data: AdditionalCellInfoData,
	/// A Unix timestamp representing when was the last time the cache was updated on the server before it was cached in the browser
	pub last_updated: i64,
	/// A Unix timestamp representing when the data was cached
	pub timestamp: i64,
}

/// The raw cache structure for a schedule day
#[derive(Debug, Deserialize, Serialize)]
pub struct HarmonogramDayData {
	/// The day the cache is bound to
	pub day: String,
	/// The cache itself (if any)
	pub cache: Option<HarmonogramDayCache>,
}

/// A structure representing the response for calling the API endpoints for specific days
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramDayResponse {
	/// The response data (if any)
	pub data: Option<HarmonogramData>,
	/// The response error (if any)
	pub error: Option<String>,
}

/// The cache structure for a schedule day.
/// This is what is saved in the browser.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramDayCache {
	/// The cache data
	pub data: HarmonogramData,
	/// A Unix timestamp representing when the data was cached
	pub timestamp: i64,
}

/// The schedule data
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramData {
	/// A two dimensional array of schedule fields (that can be None), essentially representing the schedule as a table
	pub harmonogram: Vec<Vec<Option<HarmonogramField>>>,
	/// A Unix timestamp representing when the cache was last updated on the server
	pub last_updated: i64,
}

/// A structure representing one schedule field (one schedule table cell)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmonogramField {
	/// How many columns it takes up (if any)
	pub col_span: Option<u8>,
	/// How many rows it takes up (if any)
	pub row_span: Option<u8>,
	/// Whether or not the lecture is suitable for younger audience
	pub for_younger: bool,
	/// The internal id of the lecture (if any)
	pub id: Option<String>,
	/// The lecturer's name
	pub lecturer: String,
	/// The lecture title
	pub title: String,
}

/// The additional info about a lecture
#[derive(Default, Debug, PartialEq)]
pub struct AdditionalCellInfo {
	/// The data from the API (if any)
	pub data: Option<AdditionalCellInfoData>,
	/// The warning (if any)
	pub warning: Option<String>,
	/// The error from the API (if any)
	pub error: Option<String>,
	/// A Unix timestamp representing when the cache was last updated on the server
	pub last_updated: i64,
}

impl AdditionalCellInfo {
	/// Constructs a new AdditionalCellInfo instance with the data, warning, error, and last updated fields provided
	pub fn new(data: Option<AdditionalCellInfoData>, warning: Option<String>, error: Option<String>, last_updated: i64) -> Self {
		Self { data, warning, error, last_updated }
	}
}

/// The additional data about a lecture
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AdditionalCellInfoData {
	/// The lecturer's name
	pub lecturer: String,
	/// The lecture title
	pub title: String,
	/// Whether or not the lecture is suitable for younger audience
	pub for_younger: bool,
	/// The start time of the lecture (if any)
	pub start_time: Option<String>,
	/// The end time of the lecture (if any)
	pub end_time: Option<String>,
	/// The annotation of the lecture (if any)
	pub annotation: Option<String>,
	/// The information about the lecturer (if any)
	pub lecturer_info: Option<String>,
	/// The rooms the lecture will take place at
	pub lecture_rooms: Vec<String>,
	/// The day the lecture will take place on
	pub day: String,
}

/// The response format for the `/anotace` API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponse {
	/// The data from the server
	pub data: Option<AdditionalCellInfoResponseData>,
	/// The error from the server
	pub error: Option<String>,
}

/// The data format for the `/anotace` API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponseData {
	/// The information about the lecture
	pub info: AdditionalCellInfoResponseInfo,
	/// The Unix timestamp at which the cache was last updated on the server
	pub last_updated: i64,
}

/// The information format for the `/anotace` API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalCellInfoResponseInfo {
	/// The lecture annotation (if any)
	pub annotation: Option<String>,
	/// The information about the lecturer (if any)
	pub lecturer_info: Option<String>,
}

/// The schedule data state
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HarmonogramState {
	/// The data containing the list of days and the schedule layout for each of the days (if any)
	pub data: Option<Vec<(String, HarmonogramData)>>,
	/// The error from the server (if any)
	pub error: Option<String>,
}
impl HarmonogramState {
	/// Creates a new HarmonogramState instance with the data and error provided
	pub fn new(data: Option<Vec<(String, HarmonogramData)>>, error: Option<String>) -> Self {
		Self { data, error }
	}
}

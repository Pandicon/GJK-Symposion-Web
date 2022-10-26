use web_sys::Storage;

pub fn get_local_storage() -> Option<Storage> {
	let mut res = None;
	if let Some(window) = web_sys::window() {
		if let Ok(storage) = window.local_storage() {
			res = storage;
		}
	}
	res
}

pub fn get_local_storage_key(local_storage: &Option<Storage>, key: &str) -> Option<String> {
	let mut res = None;
	if let Some(storage) = local_storage {
		let value = storage.get_item(key);
		if let Ok(value) = value {
			res = value;
		}
	}
	res
}

pub fn set_local_storage_key(local_storage: &Option<Storage>, key: &str, value: &str) -> Result<(), String> {
	let mut res = Err(String::from("No local storage provided"));
	if let Some(storage) = local_storage {
		match storage.set_item(key, value) {
			Ok(_) => res = Ok(()),
			Err(error) => res = Err(format!("{:?}", error)),
		};
	}
	res
}

pub fn remove_local_storage_key(local_storage: &Option<Storage>, key: &str) -> Result<(), String> {
	let mut res = Err(String::from("No local storage provided"));
	if let Some(storage) = local_storage {
		match storage.remove_item(key) {
			Ok(_) => res = Ok(()),
			Err(error) => res = Err(format!("{:?}", error)),
		};
	}
	res
}

pub fn raw_harmonogram_day_to_display_day(day: &str) -> &str {
	match day {
		"streda" => "$tředa",
		"ctvrtek" => "Čtvrtek",
		"patek" => "Pátek",
		_ => "Neznámý den",
	}
}

pub fn raw_harmonogram_day_to_display_day_header(day: &str) -> String {
	raw_harmonogram_day_to_display_day(day)
		.chars()
		.map(|c| match c {
			's' | 'S' => '$',
			_ => c,
		})
		.collect::<String>()
}

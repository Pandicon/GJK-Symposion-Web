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

pub fn set_local_storage_key(local_storage: &Option<Storage>, key: &str, value: &str) -> Result<(), ()> {
    let mut res = Err(());
	if let Some(storage) = local_storage {
        if storage.set_item(key, value).is_ok() {
			res = Ok(());
		};
    }
	res
}
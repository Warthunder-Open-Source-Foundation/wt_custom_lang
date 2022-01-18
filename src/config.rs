use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	pub dark_mode: bool,
	pub enable_lang: bool,

	/// WT specific items
	pub wt_path: Option<String>,
	pub blk_set: bool,
	pub lang_folder_created: bool,
	pub prompted_about_lang_perm: bool,
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			dark_mode: true,
			enable_lang: true,
			wt_path: None,
			blk_set: false,
			lang_folder_created: false,
			prompted_about_lang_perm: false,
		}
	}
}

impl Configuration {
	// pub fn is_set_up(&self) -> bool {
	// 	return self.is_wt_path_valid() && self.is_blk_setup() && self.is_lang_folder_created();
	// }
	pub fn is_wt_path_valid(&self) -> bool {
		if let Some(path) = &self.wt_path {
			if fs::read(&format!("{}/config.blk", path)).is_ok() {
				return true;
			}
		}
		false
	}
	pub fn is_blk_setup(&self) -> bool {
		if let Some(path) = &self.wt_path {
			if let Ok(file) = fs::read_to_string(&format!("{}/config.blk", path)) {
				if file.contains("testLocalization:b=yes") {
					return true;
				}
			}
		}
		false
	}
	pub fn is_lang_folder_created(&self) -> bool {
		if let Some(path) = &self.wt_path {
			return fs::read_dir(format!("{}/lang", path)).is_ok();
		}
		false
	}
	pub fn is_lang_perm_set(&self) -> bool {
		if let Some(path) = &self.wt_path {
			let test_path = format!("{}/lang/units.csv", path);
			if let Ok(bytes) = fs::read(&test_path) {
				return fs::write(&test_path, bytes).is_ok();
			}
		}
		false
	}
	pub fn is_lang_enabled(&self) -> Option<bool> {
		if let Some(path) = &self.wt_path {
			if let Ok(file) = fs::read_to_string(&format!("{}/config.blk", path)) {
				if file.contains("testLocalization:b=yes") {
					return Some(true);
				} else if file.contains("testLocalization:b=no") {
					return Some(false);
				}
			}
		}
		None
	}
}
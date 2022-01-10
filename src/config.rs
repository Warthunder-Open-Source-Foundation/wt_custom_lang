use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	pub dark_mode: bool,
	pub wt_path: Option<String>,
	pub blk_set: bool,
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			dark_mode: true,
			wt_path: None,
			blk_set: false,
		}
	}
}

impl Configuration {
	pub fn is_set_up(&self) -> bool {
		return if self.wt_path.is_some() && self.blk_set == true {
			true
		} else {
			false
		};
	}
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
}
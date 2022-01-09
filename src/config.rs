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
}
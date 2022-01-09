use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	pub dark_mode: bool,
	pub wt_path: Option<String>,
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			dark_mode: true,
			wt_path: None,
		}
	}
}
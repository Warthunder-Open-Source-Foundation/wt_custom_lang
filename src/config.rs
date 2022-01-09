use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	pub dark_mode: bool,
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			dark_mode: true,
		}
	}
}
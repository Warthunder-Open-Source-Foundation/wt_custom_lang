pub struct Cache {
	/// Left name of the buffered asset, right contents
	csv_file_buffer: (String, Vec<u8>),
}

impl Cache {
	pub fn new() -> Self {
		Self {
			csv_file_buffer: ("".to_owned(), Vec::new())
		}
	}
}

use regex::Regex;

pub struct Entry {
	pub id: Option<String>,
	pub original_english: String,
	pub new_english: String,
}

impl Entry {
	pub fn replace_all_entries(entries: Vec<Self>, file: String) -> String {
		let mut new_file = file;
		let regex_bounds = (r#"""#, r#"""#);
		for entry in entries {
			let re = Regex::new(&format!("{}{}{}", regex_bounds.0, entry.original_english, regex_bounds.1)).unwrap();
			new_file = re.replace_all(&new_file, format!("\"{}\"", &entry.new_english)).parse().unwrap();
		}
		new_file
	}
}

mod tests {
	use super::*;

	#[test]
	fn regex_confirm() {
		let entries = vec![Entry {
			id: None,
			original_english: "Text-A".to_string(),
			new_english: "Text-B".to_string()
		}];
		let old_text = r#""Text-A" "Text-C" "Text-A-1""#.to_owned();
		let new_text = Entry::replace_all_entries(entries, old_text);

		assert_eq!(r#""Text-B" "Text-C" "Text-A-1""#, new_text)
	}
}
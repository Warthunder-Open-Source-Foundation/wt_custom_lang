use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrimitiveEntry {
	pub file: String,
	pub id: Option<String>,
	pub original_english: String,
	pub new_english: String,
}

impl PrimitiveEntry {
	pub fn replace_all_entries_re(entries: Vec<Self>, file: &mut String) {
		let regex_bounds = (r#"""#, r#"""#);
		for entry in entries {
			let re = Regex::new(&format!("{}{}{}", regex_bounds.0, entry.original_english, regex_bounds.1)).unwrap();
			*file = re.replace_all(&file, format!("\"{}\"", &entry.new_english)).to_string();
		}
	}
	pub fn replace_all_entries_str(entries: Vec<Self>, file: &mut String, whole_word: bool) {
		if whole_word {
			for entry in entries {
				*file = file.replace(&format!("\"{}\"", &entry.original_english), &format!("\"{}\"", &entry.new_english));
			}
		} else {
			for entry in entries {
				*file = file.replace(&entry.original_english, &entry.new_english);
			}
		}
	}
}

mod tests {
	use crate::lang_manipulation::primitive_lang::PrimitiveEntry;

	#[test]
	fn regex_confirm() {
		let entries = vec![PrimitiveEntry {
			file: "".to_string(),
			id: None,
			original_english: "Tiger II (H) Sla.16".to_string(),
			new_english: "Tiger test".to_string(),
		}];
		let mut old_text = r#""Tiger II (H) Sla.16""#.to_owned();
		PrimitiveEntry::replace_all_entries_re(entries, &mut old_text);

		assert_eq!(r#""Tiger test""#, old_text)
	}

	#[test]
	fn str_whole_confirm() {
		let entries = vec![PrimitiveEntry {
			file: "".to_string(),
			id: None,
			original_english: "Tiger II (H) Sla.16".to_string(),
			new_english: "Tiger test".to_string(),
		}];
		let mut old_text = r#""Tiger II (H) Sla.16";"Tiger II (H) Sla.16";"Tiger II (H)";"#.to_owned();
		PrimitiveEntry::replace_all_entries_str(entries, &mut old_text, true);

		assert_eq!(r#""Tiger test";"Tiger test";"Tiger II (H)";"#, old_text)
	}
	#[test]
	fn str_partial_confirm() {
		let entries = vec![PrimitiveEntry {
			file: "".to_string(),
			id: None,
			original_english: "Tiger II (H)".to_string(),
			new_english: "Tiger test".to_string(),
		}];
		let mut old_text = r#""Tiger II (H) Sla.16";"Tiger II (H) Sla.16";"Tiger II (H)";"#.to_owned();
		PrimitiveEntry::replace_all_entries_str(entries, &mut old_text, false);

		assert_eq!(r#""Tiger test Sla.16";"Tiger test Sla.16";"Tiger test";"#, old_text)
	}
}
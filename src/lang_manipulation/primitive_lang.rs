use std::fs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::CustomLang;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrimitiveEntry {
	pub file: String,
	pub id: Option<String>,
	pub original_english: String,
	pub new_english: String,
}

impl PrimitiveEntry {
	// #[allow(dead_code)]
	// pub fn replace_all_entries_from_file_re(entries: Vec<Self>, file: &mut String) {
	// 	let regex_bounds = (r#"""#, r#"""#);
	// 	for entry in entries {
	// 		let re = Regex::new(&format!("{}{}{}", regex_bounds.0, entry.original_english, regex_bounds.1)).unwrap();
	// 		*file = re.replace_all(&file, format!("\"{}\"", &entry.new_english)).to_string();
	// 	}
	// }
	pub fn replace_all_entries_from_file_str(entries: Vec<Self>, file: &mut String, whole_word: bool) {
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
	pub fn replace_all_entries_direct_str(custom_lang: &mut CustomLang, entries: &[Self], wt_path: &str, whole_word: bool) {
		let string_to_path = |x: &str| format!("{}/lang/{}.csv", wt_path, x);


		let mut units = file_to_string(custom_lang, "units");
		let mut ui = file_to_string(custom_lang, "ui");
		let mut common_languages = file_to_string(custom_lang, "_common_languages");
		let mut menu = file_to_string(custom_lang, "menu");

		if custom_lang.prompt_error.err_value.is_some() {
			return;
		}

		let format = if whole_word {
			|x: &str| format!("\"{}\"", x)
		} else {
			|x: &str| x.to_owned()
		};
		for entry in entries {
			match entry.file.as_str() {
				"units" => {
					units = units.replace(&format(&entry.new_english), &format(&entry.original_english));
				}
				"ui" => {
					ui = ui.replace(&format(&entry.new_english), &format(&entry.original_english));
				}
				"_common_languages" => {
					common_languages = common_languages.replace(&format(&entry.new_english), &format(&entry.original_english));
				}
				"menu" => {
					menu = menu.replace(&format(&entry.new_english), &format(&entry.original_english));
				}
				_ => {
					let mut file = file_to_string(custom_lang, &entry.file);
					file = file.replace(&format(&entry.new_english), &format(&entry.original_english));
					if let Err(error) = fs::write(string_to_path(&entry.file), file) {
						custom_lang.prompt_error.err_value = Some(error.to_string());
					}
				}
			}
		}
		string_to_file(custom_lang, &string_to_path("units"), &units);
		string_to_file(custom_lang, &string_to_path("ui"), &ui);
		string_to_file(custom_lang, &string_to_path("_common_languages"), &common_languages);
		string_to_file(custom_lang, &string_to_path("menu"), &menu);
	}
}

fn string_to_file(custom_lang: &mut CustomLang, path: &str, file: &str) {
	match serde_json::to_string(file) {
		Ok(bin) => {
			match fs::write(path, bin) {
				Ok(_) => {}
				Err(error) => {
					custom_lang.prompt_error.err_value = Some(error.to_string());
					return;
				}
			}
		}
		Err(error) => {
			custom_lang.prompt_error.err_value = Some(error.to_string());
			return;
		}
	}
}

fn file_to_string(custom_lang: &mut CustomLang, path: &str) -> String {
	let string_to_path = |x: &str| format!("{}/lang/{}.csv", path, x);

	return match fs::read_to_string(string_to_path(path)) {
		Ok(value) => { value }
		Err(error) => {
			custom_lang.prompt_error.err_value = Some(error.to_string());
			"".to_owned()
		}
	};
}

mod tests {
	#[test]
	fn regex_confirm() {
		let entries = vec![PrimitiveEntry {
			file: "".to_string(),
			id: None,
			original_english: "Tiger II (H) Sla.16".to_string(),
			new_english: "Tiger test".to_string(),
		}];
		let mut old_text = r#""Tiger II (H) Sla.16""#.to_owned();
		PrimitiveEntry::replace_all_entries_from_file_re(entries, &mut old_text);

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
		PrimitiveEntry::replace_all_entries_from_file_str(entries, &mut old_text, true);

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
		PrimitiveEntry::replace_all_entries_from_file_str(entries, &mut old_text, false);

		assert_eq!(r#""Tiger test Sla.16";"Tiger test Sla.16";"Tiger test";"#, old_text)
	}
}
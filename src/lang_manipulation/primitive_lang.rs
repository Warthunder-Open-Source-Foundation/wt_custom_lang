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
	pub fn replace_all_entries(entries: Vec<Self>, file: &mut String) {
		let regex_bounds = (r#"""#, r#"""#);
		for entry in entries {
			let re = Regex::new(&format!("{}{}{}", regex_bounds.0, entry.original_english, regex_bounds.1)).unwrap();
			*file = re.replace_all(&file, format!("\"{}\"", &entry.new_english)).to_string();
		}
	}
}

mod tests {
	#[test]
	fn regex_confirm() {
		let entries = vec![PrimitiveEntry {
			id: None,
			original_english: "MiG-23MLD".to_string(),
			new_english: "MiG-23MALD".to_string(),
		}];
		let mut old_text = r#""mig_23mld_shop";"MiG-23MLD";"MiG-23MLD";"MiG-23MLD""#.to_owned();
		PrimitiveEntry::replace_all_entries(entries, &mut old_text);

		assert_eq!(r#""mig_23mld_shop";"MiG-23MALD";"MiG-23MALD";"MiG-23MALD""#, old_text)
	}
}
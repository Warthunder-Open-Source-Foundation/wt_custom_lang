// use std::fs;
//
// use serde::{Deserialize, Serialize};
//
// const HEADER: &str = r#""<ID|readonly|noverify>";"<English>";"<French>";"<Italian>";"<German>";"<Spanish>";"<Russian>";"<Polish>";"<Czech>";"<Turkish>";"<Chinese>";"<Japanese>";"<Portuguese>";"<Ukrainian>";"<Serbian>";"<Hungarian>";"<Korean>";"<Belarusian>";"<Romanian>";"<TChinese>";"<HChinese>";"<Comments>";"<max_chars>""#;
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Unit {
// 	pub id: String,
// 	pub english: String,
// 	// french: String,
// 	// italian: String,
// 	// german: String,
// 	// spanish: String,
// 	// russian: String,
// 	// polish: String,
// 	// czech: String,
// 	// turkish: String,
// 	// chinese: String,
// 	// japanese: String,
// 	// portuguese: String,
// 	// ukrainian: String,
// 	// serbian: String,
// 	// hungarian: String,
// 	// korean: String,
// 	// belarusian: String,
// 	// romanian: String,
// 	// t_chinese: String,
// 	// h_chinese: String,
// }
//
// impl Unit {
// 	pub fn from_file(path: &str) -> Vec<Self> {
// 		let mut file = fs::read_to_string(path).unwrap();
//
// 		file = file.replace("\"<", "\"").replace(">\"", "\"");
//
// 		let mut units: Vec<Unit> = Vec::new();
//
// 		let mut rdr = csv::ReaderBuilder::new()
// 			.delimiter(b';')
// 			.quoting(true)
// 			.has_headers(true)
// 			.from_reader(file.as_bytes());
//
// 		for entry in rdr.records() {
// 			if let Ok(result) = entry {
// 				units.push(Unit {
// 					id: result[0].to_owned(),
// 					english: result[1].to_owned(),
// 				})
// 			}
// 		}
// 		units
// 	}
// 	pub fn convert_to_csv(_entries: Vec<Self>) {
// 		let mut wtr = csv::Writer::from_path(fs::write("new_lang.csv", "").unwrap()).unwrap();
// 	}
// }
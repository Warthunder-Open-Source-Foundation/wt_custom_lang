use std::fs;
use lazy_static::lazy_static;
use duckstore::{DirType, PathConfig, ResolvedPaths};


use crate::{CONFIG_NAME, PrimitiveEntry};

pub const LANG_ENTRIES: PathConfig = PathConfig {
	project_prefix: CONFIG_NAME,
	sub_folder: "lang",
	file_name: "entries.bin",
	dir_type: &DirType::Data,
};

lazy_static! {
 pub static ref LANG_PATH: ResolvedPaths<'static> = {
		match LANG_ENTRIES.resolve() {
			Ok(x) => x,
			// Unsafe unwrap as this error cannot be recovered at runtime
			Err(err) => panic!("{err}"),
		}
    };
}

pub const READ_PRIMITIVE: fn() -> Vec<PrimitiveEntry> = || {
	let bin = fs::read_to_string(&LANG_PATH.constructed_path).unwrap();
	serde_json::from_str(&bin).unwrap()
};

pub const WRITE_PRIMITIVE: fn(&Vec<PrimitiveEntry>) = |x: &Vec<PrimitiveEntry>|{
	let bin = serde_json::to_string(x).unwrap();
	fs::write(&LANG_PATH.constructed_path, &bin).unwrap();
};
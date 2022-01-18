#![windows_subsystem = "windows"]

use std::{fs, panic};
use duckstore::{DirType, PathConfig, ResolvedPaths};

use eframe::{NativeOptions, run_native};
use eframe::egui::Vec2;
use lazy_static::lazy_static;

use app::custom_lang::CustomLang;
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;

mod config;
mod lang_manipulation;
mod app;

const REPO_URL: &str = "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master";

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

const LANG_ENTRIES: PathConfig = PathConfig {
	project_prefix: CONFIG_NAME,
	sub_folder: "lang",
	file_name: "entries.bin",
	dir_type: &DirType::Data,
};

lazy_static! {
 static ref LANG_PATH: ResolvedPaths<'static> = {
		match LANG_ENTRIES.resolve() {
			Ok(x) => x,
			// Unsafe unwrap as this error cannot be recovered at runtime
			Err(err) => panic!("{err}"),
		}
    };
}

const READ_PRIMITIVE: fn() -> Vec<PrimitiveEntry> = || {
	let bin = fs::read_to_string(&LANG_PATH.constructed_path).unwrap();
	serde_json::from_str(&bin).unwrap()
};

const WRITE_PRIMITIVE: fn(&Vec<PrimitiveEntry>) = |x: &Vec<PrimitiveEntry>|{
	let bin = serde_json::to_string(x).unwrap();
	fs::write(&LANG_PATH.constructed_path, &bin).unwrap();
};

pub fn main() {
	#[cfg(not(debug_assertions))]
		panic::set_hook(Box::new(|panic_info| {
		println!("{}", panic_info);
		let dir = directories::BaseDirs::new().unwrap();
		let data_dir = dir.data_dir().to_str().unwrap();
		let final_path = &format!("{}/{}/error/{}.log", data_dir, CONFIG_NAME, chrono::offset::Local::now().format("%Y-%m-%d--%H-%M-%S"));

		println!("{}", final_path);

		fs::create_dir_all(&format!("{}/{}/error", data_dir, CONFIG_NAME));

		match fs::write(final_path, panic_info.to_string()) {
			Ok(_) => {
				println!("Error log written to {}", final_path);
			}
			Err(err) => {
				println!("Failed to save error log due to:  {}", err);
			}
		}
	}));

	if fs::read(&LANG_PATH.constructed_path).is_err() {
		fs::write(&LANG_PATH.constructed_path, b"[]").unwrap();
	}

	let app = CustomLang::new();

	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));

	run_native(Box::new(app), window_options);
}
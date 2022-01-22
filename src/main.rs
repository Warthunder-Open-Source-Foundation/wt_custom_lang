#![windows_subsystem = "windows"]

use std::{fs};

use eframe::{NativeOptions, run_native};
use eframe::egui::Vec2;


use app::custom_lang::CustomLang;
use crate::app::prompts::prompt_for_backup::BACKUP_ENTRY_STORAGE;
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
use crate::local_storage::entries::LANG_PATH;

mod config;
mod lang_manipulation;
pub mod app;
pub mod local_storage;

const REPO_URL: &str = "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master";

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

pub fn main() {
	// #[cfg(not(debug_assertions))]
		// {
		// 	std::panic::set_hook(Box::new(|panic_info| {
		// 		let err_notification = || {
		// 			// Error dropped as there is quite literally nothing that can be done at this point
		// 			let _ = Notification::new()
		// 				.summary("WT-custom-lang exited unexpectedly")
		// 				.body("if this issue keeps occurring please open an issue")
		// 				.show();
		// 		};
		//
		// 		if let Some(dir) = directories::BaseDirs::new() {
		// 			if let Some(data_dir) = dir.data_dir().to_str() {
		// 				let final_path = &format!("{}/{}/error/{}.log", data_dir, CONFIG_NAME, chrono::offset::Local::now().format("%Y-%m-%d--%H-%M-%S"));
		//
		// 				let _ = fs::create_dir_all(&format!("{}/{}/error", data_dir, CONFIG_NAME));
		//
		// 				match fs::write(final_path, panic_info.to_string()) {
		// 					Ok(_) => {
		// 						println!("Error log written to {}", final_path);
		// 					}
		// 					Err(err) => {
		// 						println!("Failed to save error log due to:  {}", err);
		// 					}
		// 				}
		// 			} else {
		// 				err_notification();
		// 			}
		// 		} else {
		// 			err_notification();
		// 		}
		// 	}));
		// }

	lazy_static::initialize(&LANG_PATH);

	if fs::read(&LANG_PATH.constructed_path).is_err() {
		// "Save" unwrap as the panic handler will catch this, most likely will be a OS permission collision
		match fs::write(&LANG_PATH.constructed_path, b"[]") {
			Ok(_) => {}
			Err(err) => {
				panic!("{:?}", err);
			}
		};
	}

	if fs::read(&BACKUP_ENTRY_STORAGE()).is_err() {
		// "Save" unwrap as the panic handler will catch this, most likely will be a OS permission collision
		match fs::write(&BACKUP_ENTRY_STORAGE(), b"[]") {
			Ok(_) => {}
			Err(err) => {
				panic!("{:?}", err);
			}
		};
	}

	let app = CustomLang::new();

	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));

	run_native(Box::new(app), window_options);
}
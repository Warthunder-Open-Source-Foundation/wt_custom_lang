#![windows_subsystem="windows"]

use std::{fs, panic};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use eframe::{NativeOptions, run_native};
use eframe::egui::Vec2;
use execute::{command, Execute};
use sysinfo::{System, SystemExt};

use app::custom_lang::CustomLang;

mod config;
mod lang_manipulation;
mod app;

const REPO_URL: &str = "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master";

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

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

	let app = CustomLang::new();

	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));

	run_native(Box::new(app), window_options);
}
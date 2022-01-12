use std::process::exit;

use eframe::{NativeOptions, run_native};
use eframe::egui::Vec2;
use sysinfo::{System, SystemExt};

use app::custom_lang::CustomLang;

mod config;
mod lang_manipulation;
mod app;

const REPO_URL: &str = "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master";

pub fn main() {
	let app = CustomLang::new();

	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));

	run_native(Box::new(app), window_options);
}
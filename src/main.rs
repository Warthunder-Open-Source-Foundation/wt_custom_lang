use eframe::egui::Vec2;
use eframe::{NativeOptions, run_native};

use crate::custom_lang::CustomLang;

pub mod custom_lang;
pub mod lang;

pub fn main() {
	let app = CustomLang;
	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));
	run_native(Box::new(app), window_options);
}
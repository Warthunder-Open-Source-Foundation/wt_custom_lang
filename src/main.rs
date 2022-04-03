#![windows_subsystem = "windows"]

use std::fs;

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
mod cache;

const REPO_URL: &str = "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master";

const CONFIG_NAME: &str = "wt_custom_lang"; //DO NOT CHANCE UNLESS ABSOLUTELY NECESSARY

const TRACKED_FILES: [&str; 54] = [
	"_common_languages.csv",
	"menu_debriefing.csv",
	"missions_pilots.csv",
	"ui.csv",
	"_keyboard.csv",
	"menu_discount_description.csv",
	"missions_single.csv",
	"units.csv",
	"_legal.csv",
	"menu_events.csv",
	"missions_training.csv",
	"units_modifications.csv",
	"_missing.csv",
	"menu_multiplayer.csv",
	"missions_tutorial.csv",
	"units_weaponry.csv",
	"_online.csv",
	"menu_options.csv",
	"missions_versus.csv",
	"unlocks_achievements.csv",
	"benchmark.csv",
	"missions_briefing.csv",
	// "pc_UiMessages.csv",
	"unlocks_attachables.csv",
	"controls.csv",
	"missions_campaign.csv",
	"shop.csv",
	"unlocks_challenges.csv",
	"encyclopedia.csv",
	"missions_chapters.csv",
	"speech_coral_sea.csv",
	"unlocks_conditions.csv",
	"encyclopedia_tips.csv",
	"missions_debriefings.csv",
	"speech_guadalcanal.csv",
	"unlocks_decals.csv",
	"localization.blk",
	"missions_dynamic.csv",
	"speech_honolulu.csv",
	"unlocks_medals.csv",
	"matching.csv",
	"missions_fails.csv",
	"speech_midway.csv",
	"unlocks_skins.csv",
	"menu.csv",
	"missions_hints.csv",
	"speech_tutorial.csv",
	"unlocks_streaks.csv",
	"menu_chat.csv",
	"missions_locations.csv",
	"speech_video.csv",
	"worldwar.csv",
	"menu_clan.csv",
	"missions_objectives.csv",
	"speech_wake_island.csv",
];

pub fn main() {
	#[cfg(not(debug_assertions))]
	{
		std::panic::set_hook(Box::new(|panic_info| {
			let err_notification = || {
				// Error dropped as there is quite literally nothing that can be done at this point
				let _ = notify_rust::Notification::new().summary("WT-custom-lang exited unexpectedly").body("if this issue keeps occurring please open an issue").show();
			};

			if let Some(dir) = directories::BaseDirs::new() {
				if let Some(data_dir) = dir.data_dir().to_str() {
					let final_path = &format!("{}/{}/error/{}.log", data_dir, CONFIG_NAME, chrono::offset::Local::now().format("%Y-%m-%d--%H-%M-%S"));

					let _ = fs::create_dir_all(&format!("{}/{}/error", data_dir, CONFIG_NAME));

					match fs::write(final_path, panic_info.to_string()) {
						Ok(_) => {
							println!("Error log written to {}", final_path);
						}
						Err(err) => {
							println!("Failed to save error log due to:  {}", err);
						}
					}
				} else {
					err_notification();
				}
			} else {
				err_notification();
			}
		}));
	}

	lazy_static::initialize(&LANG_PATH);

	if fs::read(&LANG_PATH.constructed_path).is_err() {
		// "Save" unwrap as the panic handler will catch this, most likely will be a OS permission collision
		match fs::write(&LANG_PATH.constructed_path, b"[]") {
			Ok(_) => {}
			Err(err) => {
				panic!("{}:{} {:?}", line!(), column!(), err);
			}
		};
	}

	if fs::read(&BACKUP_ENTRY_STORAGE()).is_err() {
		// "Save" unwrap as the panic handler will catch this, most likely will be a OS permission collision
		match fs::write(&BACKUP_ENTRY_STORAGE(), b"[]") {
			Ok(_) => {}
			Err(err) => {
				panic!("{}:{} {:?}", line!(), column!(), err);
			}
		};
	}

	let app = CustomLang::new();

	let mut window_options = NativeOptions::default();
	window_options.initial_window_size = Some(Vec2::new(900.0, 600.0));

	run_native(Box::new(app), window_options);
}
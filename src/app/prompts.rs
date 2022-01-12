use std::{fs, thread};
use std::process::Command;

use eframe::egui::*;
use eframe::egui::Button;
use eframe::egui::Label;
use execute::Execute;
use rfd::FileDialog;


use crate::lang_manipulation::primitive_lang::PrimitiveEntry;

use crate::{CustomLang, REPO_URL};

impl CustomLang {
	pub(crate) fn prompt_for_status(&mut self, ctx: &CtxRef) {
		Window::new("Config status").show(ctx, |ui| {
			if self.config.is_wt_path_valid() {
				ui.add(Label::new(RichText::new(format!("WT path is defined and working ✅")).color(Color32::from_rgb(0, 255, 0))));
			}
			if self.config.is_blk_setup() {
				ui.add(Label::new(RichText::new(format!("Config.blk is configured properly ✅")).color(Color32::from_rgb(0, 255, 0))));
			}
			if self.config.is_lang_folder_created() {
				ui.add(Label::new(RichText::new(format!("Lang folder was created ✅")).color(Color32::from_rgb(0, 255, 0))));
			}
			// #[cfg(target_os = "windows")]
			if self.config.is_lang_perm_set() {
				ui.add(Label::new(RichText::new(format!("Lang permission is set ✅")).color(Color32::from_rgb(0, 255, 0))));
			}
			if ui.add(Button::new("Close")).clicked() {
				self.status_menu = false;
			}
		});
	}
	pub(crate) fn prompt_for_wt_path(&mut self, ctx: &CtxRef) {
		Window::new("First time setup").show(ctx, |ui| {
			ui.add(Label::new("Select WarThunder installation folder"));
			let select_button = ui.add(Button::new(RichText::new("Choose path").text_style(TextStyle::Body)));
			ui.add(Hyperlink::from_label_and_url("Where the game might be installed", format!("{}/guide/install_folder.md", REPO_URL)));

			if select_button.clicked() {
				if let Some(path) = FileDialog::new().pick_folder() {
					if fs::read(&format!("{}/config.blk", path.to_str().unwrap())).is_ok() {
						self.config.wt_path = Some(path.to_str().unwrap().to_owned());
						ui.add(Label::new(format!("Path {} successfully selected", path.to_str().unwrap())));
					} else {
						ui.add(Label::new(format!("Path {} is invalid", path.to_str().unwrap())));
					}
				}
			}
		});
	}
	pub(crate) fn prompt_for_config_blk(&mut self, ctx: &CtxRef) {
		Window::new("Configuring the config.blk file").show(ctx, |ui| {
			let blk_path = format!("{}/config.blk", self.config.wt_path.as_ref().unwrap());
			let config_blk = fs::read_to_string(&blk_path).unwrap();

			if !config_blk.contains("testLocalization:b=yes") {
				if ui.add(Button::new("Configure config.blk")).clicked() {
					// Using this non-conforming strategy of editing the file, as it uses a undefined file format
					let debug_loc = config_blk.split_at(config_blk.find("debug{").unwrap() + 7);
					let new = format!("{}\n{}{}", debug_loc.0, "  testLocalization:b=yes", debug_loc.1);

					if fs::write(&blk_path, new).is_ok() {
						self.config.blk_set = true;
					}
				}
				if ui.add(Button::new("I already configured config.blk")).clicked() {
					self.config.blk_set = true;
				}
			} else {
				self.config.blk_set = true;
			}
		});
	}
	pub(crate) fn prompt_for_lang_folder(&mut self, ctx: &CtxRef) {
		Window::new("Steps for generating the lang folder").show(ctx, |ui| {
			if ui.add(Button::new(RichText::new("Launch game").text_style(TextStyle::Heading))).clicked() {
				// Cloning as the thread consumes the String entirely
				let path = self.config.wt_path.as_ref().unwrap().clone();

				#[cfg(target_os = "windows")] let format_path = format!("{}/launcher.exe", path);

				#[cfg(target_os = "linux")] let format_path = format!("{}/launcher", path);

				// Spawning loose thread as application completely stalls as long as launcher.exe lives
				thread::spawn(move || {
					Command::new(format_path).execute();
				});
			}

			ui.add_space(20.0);

			if ui.add(Button::new(RichText::new("Check if it was created").text_style(TextStyle::Heading))).clicked() {
				if fs::read_dir(format!("{}/lang", self.config.wt_path.as_ref().unwrap())).is_ok() {
					self.config.lang_folder_created = true;
				}
			}
		});
	}
	pub(crate) fn prompt_for_entry(&mut self, ctx: &CtxRef) {
		Window::new("Adding a new entry").show(ctx, |ui| {
			let mut original = self.add_csv_entry.clone().unwrap();
			ui.add(TextEdit::singleline(&mut original.0));
			ui.add(TextEdit::singleline(&mut original.1));

			self.add_csv_entry = Some(original);

			if ui.add(Button::new(RichText::new("Create!").text_style(TextStyle::Heading))).clicked() {
				let path = format!("{}/lang/units.csv", self.config.wt_path.as_ref().unwrap());
				let mut file = fs::read_to_string(&path).unwrap();

				let entry = PrimitiveEntry {
					id: None,
					original_english: self.add_csv_entry.as_ref().unwrap().0.trim().to_string(),
					new_english: self.add_csv_entry.as_ref().unwrap().1.trim().to_string(),
				};

				PrimitiveEntry::replace_all_entries(vec![entry.clone()], &mut file);

				if fs::write(&path, file).is_ok() {
					let mut old: Vec<PrimitiveEntry> = serde_json::from_str(&self.config.primitive_entries).unwrap();
					old.push(entry);
					self.config.primitive_entries = serde_json::to_string(&old).unwrap();
				}
				self.add_csv_entry = None;
			}
			if ui.add(Button::new(RichText::new("Cancel").text_style(TextStyle::Heading))).clicked() {
				self.add_csv_entry = None;
			}
		});
	}
	pub(crate) fn prompt_lang_file_warn(&mut self, ctx: &CtxRef) {
		Window::new("Setting lang folder permissions").show(ctx, |ui| {
			if ui.add(Button::new(RichText::new("Done!").text_style(TextStyle::Heading))).clicked() {
				self.config.prompted_about_lang_perm = true;
			}
			ui.add(Hyperlink::from_label_and_url("I dont know how to do that", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/guide/windows_lang_permission.md"));
		});
	}
}
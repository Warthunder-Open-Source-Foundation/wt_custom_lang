use std::{fs, thread};
use std::process::Command;

use eframe::egui::*;
use eframe::egui::Button;
use eframe::egui::Label;
use execute::Execute;
use rfd::FileDialog;

use crate::{CustomLang, REPO_URL};

impl CustomLang {
	pub fn prompt_for_status(&mut self, ctx: &CtxRef) {
		Window::new("Config status").anchor(Align2::CENTER_CENTER, Vec2::new(0.0,0.0)).show(ctx, |ui| {
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
		Window::new("First time setup").anchor(Align2::CENTER_CENTER, Vec2::new(0.0,0.0)).show(ctx, |ui| {
			ui.add(Label::new("Select WarThunder installation folder"));
			let select_button = ui.add(Button::new(RichText::new("Choose path").text_style(TextStyle::Body)));
			ui.add(Hyperlink::from_label_and_url("Where the game might be installed", format!("{}/guide/install_folder.md", REPO_URL)));

			if select_button.clicked() {
				if let Some(path) = FileDialog::new().pick_folder() {
					if let Some(path) = path.to_str() {
						if fs::read(&format!("{}/config.blk", path)).is_ok() {
							self.config.wt_path = Some(path.to_owned());
							ui.add(Label::new(format!("Path {} successfully selected", path)));
						} else {
							ui.add(Label::new(format!("Path {} is invalid", path)));
						}
					} else {
						self.prompt_error.err_value = Some("Could not convert chosen path to real system path".to_owned());
						return;
					}
				}
			}
		});
	}
	pub fn prompt_for_config_blk(&mut self, ctx: &CtxRef) {
		Window::new("Configuring the config.blk file").anchor(Align2::CENTER_CENTER, Vec2::new(0.0,0.0)).show(ctx, |ui| {
			if let Some(wt_path) = self.config.wt_path.as_ref() {
				let blk_path = format!("{}/config.blk", wt_path);
				match fs::read_to_string(&blk_path) {
					Ok(config_blk) => {
						if !config_blk.contains("testLocalization:b=yes") {
							if ui.add(Button::new("Configure config.blk")).clicked() {
								// Using this non-conforming strategy of editing the file, as it uses a undefined file format
								if let Some(debug_num) = config_blk.find("debug{") {
									let debug_loc = config_blk.split_at(debug_num + 7);
									let new = format!("{}\n{}{}", debug_loc.0, "  testLocalization:b=yes", debug_loc.1);

									if fs::write(&blk_path, new).is_ok() {
										self.config.blk_set = true;
									}
								} else {
									self.prompt_error.err_value = Some("Failed to find \"debug{{\" in config.blk, it might be bricked".to_owned());
									return;
								}
							}
							if ui.add(Button::new("I already configured config.blk")).clicked() {
								self.config.blk_set = true;
							}
						} else {
							self.config.blk_set = true;
						}
					}
					Err(err) => {
						self.prompt_error.err_value = Some(err.to_string());
						return;
					}
				}
			}
		});
	}
	pub fn prompt_for_lang_folder(&mut self, ctx: &CtxRef) {
		Window::new("Launching the game").anchor(Align2::CENTER_CENTER, Vec2::new(0.0,0.0)).show(ctx, |ui| {
			ui.horizontal(|ui|{
				if ui.add(Button::new(RichText::new("Launch game ⬈").text_style(TextStyle::Heading))).clicked() {
					// Cloning as the thread consumes the String entirely
					if let Some(path) = self.config.wt_path.as_ref() {
						#[cfg(target_os = "windows")] let format_path = format!("{}/launcher.exe", path);

						#[cfg(target_os = "linux")] let format_path = format!("{}/launcher", path);

						// Spawning loose thread as application completely stalls as long as launcher.exe lives
						thread::spawn(move || {
							// Not catching as the process will be orphaned
							let _ = Command::new(format_path).execute();
						});
					} else {
						self.prompt_error.err_value = Some("No WT path is set, but at this point in time it should be".to_owned());
						return;
					}
				}

				ui.add_space(20.0);

				if ui.add(Button::new(RichText::new("Verify folder").text_style(TextStyle::Heading))).clicked() {
					if let Some(path) = self.config.wt_path.as_ref() {
						if fs::read_dir(format!("{}/lang", path)).is_ok() {
							self.config.lang_folder_created = true;
						}
					} else {
						self.prompt_error.err_value = Some("No WT path is set, but at this point in time it should be".to_owned());
						return;
					}
				}
			});
		});
	}
	#[cfg(windows)]
	pub fn prompt_lang_file_warn(&mut self, ctx: &CtxRef) {
		Window::new("Setting up permissions").anchor(Align2::CENTER_CENTER, Vec2::new(0.0,0.0)).show(ctx, |ui| {
			ui.horizontal(|ui|{
				ui.add(Hyperlink::from_label_and_url(RichText::new("How to set up proper permissions").text_style(TextStyle::Heading), "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/guide/windows_lang_permission.md"));
				if ui.add(Button::new(RichText::new("Done!").text_style(TextStyle::Heading))).clicked() {
					self.config.prompted_about_lang_perm = true;
				}
			});
		});
	}
}
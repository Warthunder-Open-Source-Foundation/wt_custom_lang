use std::{fs, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;


use eframe::egui::*;
use eframe::egui::Button;
use eframe::epi::{App, Frame, Storage};
use eframe::{egui};
use eframe::egui::FontFamily::Proportional;

use eframe::egui::TextStyle::{Body, Heading};
use eframe::egui::Label;
use execute::Execute;
use rfd::FileDialog;

use crate::config::Configuration;
use crate::primitive_lang::PrimitiveEntry;
use crate::REPO_URL;

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

pub struct CustomLang {
	pub config: Configuration,
	pub status_menu: bool,
	pub add_csv_entry: Option<(String, String)>,
}

impl App for CustomLang {
	fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
		if self.config.dark_mode {
			ctx.set_visuals(Visuals::dark());
		} else {
			ctx.set_visuals(Visuals::light());
		}
		match () {
			_ if self.config.wt_path.is_none() => {
				self.prompt_for_wt_path(ctx);
				confy::store(CONFIG_NAME, &self.config).unwrap();
			}
			_ if !self.config.blk_set => {
				self.prompt_for_config_blk(ctx);
				confy::store(CONFIG_NAME, &self.config).unwrap();
			}
			_ if !self.config.lang_folder_created => {
				self.prompt_for_lang_folder(ctx);
				confy::store(CONFIG_NAME, &self.config).unwrap();
			}
			_ if self.status_menu => {
				self.prompt_for_status(ctx);
			}
			_ if self.add_csv_entry.is_some() => {
				self.prompt_for_entry(ctx);
				confy::store(CONFIG_NAME, &self.config).unwrap();
			}
			#[cfg(windows)]
			_ if !self.config.prompted_about_lang_perm => {
				self.prompt_lang_file_warn(ctx);
				confy::store(CONFIG_NAME, &self.config).unwrap();
			}
			_ => {}
		}
		self.render_header_bar(ctx, frame);
		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
				if ui.add(Button::new("Add new entry")).clicked() {
					self.add_csv_entry = Some(("".to_owned(), "".to_owned()));
				}
				ui.add_space(15.0);
				let mut prim_array: Vec<PrimitiveEntry> = serde_json::from_str(&self.config.primitive_entries).unwrap();
				for (i, primitive_entry) in prim_array.iter().enumerate() {
					ui.add(Label::new(RichText::new(format!("{} changed to {}", primitive_entry.original_english, primitive_entry.new_english))));
					if ui.add(Button::new(RichText::new("Undo").color(Color32::from_rgb(255, 0,0)))).clicked() {
						let path = format!("{}/lang/units.csv", self.config.wt_path.as_ref().unwrap());
						let mut file = fs::read_to_string(&path).unwrap();

						let entry = PrimitiveEntry {
							id: None,
							original_english: primitive_entry.new_english.clone(),
							new_english: primitive_entry.original_english.clone(),
						};

						PrimitiveEntry::replace_all_entries(vec![entry.clone()], &mut file);

						if fs::write(&path, file).is_ok() {
							let mut old: Vec<PrimitiveEntry> = serde_json::from_str(&self.config.primitive_entries).unwrap();
							old.remove(i);
							self.config.primitive_entries = serde_json::to_string(&old).unwrap();
						}
					}
					ui.add_space(5.0);
				}
			});
			render_footer(ctx);
		});
	}

// fn save(&mut self, _storage: &mut dyn Storage) {
// 	todo!()
// }

// fn on_exit(&mut self) {
// 	todo!()
// }

	fn setup(&mut self, ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn Storage>) {
		// Run this first -------------------------------------------------------------------------------------------
		let mut font_def = FontDefinitions::default();
		font_def.font_data.insert("RobotoMono".to_owned(), FontData::from_owned(include_bytes!("../fonts/roboto_mono/static/RobotoMono-Medium.ttf").to_vec()));
		font_def.family_and_size.insert(Heading, (FontFamily::Proportional, 30.0));
		font_def.family_and_size.insert(Body, (FontFamily::Proportional, 20.0));
		font_def.fonts_for_family.get_mut(&Proportional).unwrap().insert(0, "RobotoMono".to_owned());
		ctx.set_fonts(font_def);
		// Run this first -------------------------------------------------------------------------------------------
	}


	fn name(&self) -> &str {
		"Warthunder custom lang-files"
	}

// fn warm_up_enabled(&self) -> bool {
// 	todo!()
// }
//
// fn auto_save_interval(&self) -> Duration {
// 	todo!()
// }
//
// fn max_size_points(&self) -> Vec2 {
// 	todo!()
// }
//
// fn clear_color(&self) -> Rgba {
// 	todo!()
// }
//
// fn persist_native_window(&self) -> bool {
// 	todo!()
// }
//
// fn persist_egui_memory(&self) -> bool {
// 	todo!()
// }
}

impl CustomLang {
	pub fn new() -> Self {
		let config: Configuration = confy::load(CONFIG_NAME).unwrap_or_default();
		Self {
			config,
			status_menu: false,
			add_csv_entry: None,
		}
	}
	fn render_header_bar(&mut self, ctx: &CtxRef, frame: &Frame) {
		TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.add_space(10.);
			egui::menu::bar(ui, |ui| {
				ui.with_layout(Layout::left_to_right(), |ui| {
					ui.add(Hyperlink::from_label_and_url("📓 How to use", format!("{}/guide/how_to_use.md", REPO_URL)));
				});
				ui.with_layout(Layout::right_to_left(), |ui| {
					// let close_btn = ui.add(Button::new("❌").text_style(TextStyle::Body));

					if ui.add(Button::new(RichText::new("🔄 Reset configuration").text_style(TextStyle::Body))).clicked() {
						confy::store(CONFIG_NAME, Configuration::default()).unwrap();
						frame.quit();
					}

					if ui.add(Button::new(RichText::new("Status").text_style(TextStyle::Body))).clicked() {
						self.status_menu = !self.status_menu;
					}

					if ui.add(Button::new(if self.config.dark_mode { RichText::new("☀").text_style(TextStyle::Body) } else { RichText::new("🌙").text_style(TextStyle::Body) })).clicked() {
						self.config.dark_mode = !self.config.dark_mode;
					}
				});
			});
			ui.add_space(10.);
		});
	}
	fn prompt_for_status(&mut self, ctx: &CtxRef) {
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
			if ui.add(Button::new("Close")).clicked() {
				self.status_menu = false;
			}
		});
	}
	fn prompt_for_wt_path(&mut self, ctx: &CtxRef) {
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
	fn prompt_for_config_blk(&mut self, ctx: &CtxRef) {
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
	fn prompt_for_lang_folder(&mut self, ctx: &CtxRef) {
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
	fn prompt_for_entry(&mut self, ctx: &CtxRef) {
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
	fn prompt_lang_file_warn(&mut self, ctx: &CtxRef) {
		Window::new("Setting lang folder permissions").show(ctx, |ui| {
			if ui.add(Button::new(RichText::new("Done!").text_style(TextStyle::Heading))).clicked() {
				self.config.prompted_about_lang_perm = true;
			}
			ui.add(Hyperlink::from_label_and_url("I dont know how to do that", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/guide/windows_lang_permission.md"));
		});
	}
}

fn render_footer(ctx: &CtxRef) {
	TopBottomPanel::bottom("footer").show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("Report bug", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/issues/new"));
			ui.add_space(10.0)
		})
	});
}
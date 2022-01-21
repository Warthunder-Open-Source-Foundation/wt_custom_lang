use std::fs;
use std::process::exit;

use eframe::egui;
use eframe::egui::{Button, CentralPanel, Color32, CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Layout, RichText, ScrollArea, TextStyle, TopBottomPanel, Visuals};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::Label;
use eframe::egui::TextStyle::{Body, Heading};
use eframe::epi::{App, Frame, Storage};

use crate::app::prompts::prompt_for_entry::{EMPTY_BEFORE_AFTER, LangType, PromptForEntry};
use crate::config::Configuration;
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
use crate::local_storage::entries::{READ_PRIMITIVE, WRITE_PRIMITIVE};
use crate::{CONFIG_NAME, REPO_URL};
use crate::app::prompts::prompt_error::AppError;
use crate::app::prompts::prompt_for_backup::PromptForBackup;

pub struct CustomLang {
	pub config: Configuration,
	pub status_menu: bool,
	pub prompt_for_backup: PromptForBackup,
	pub prompt_for_entry: PromptForEntry,
	pub prompt_error: AppError,
}

pub const STORE_CONF: fn(config: &Configuration) = |config| {
	if let Err(err) = confy::store(CONFIG_NAME, config) {
		panic!("Failed to write to configuration file, this error will be discontinued soon");
	}
};

impl App for CustomLang {
	fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
		if self.config.dark_mode {
			ctx.set_visuals(Visuals::dark());
		} else {
			ctx.set_visuals(Visuals::light());
		}
		if self.prompt_error.err_value.is_some() {
			self.prompt_error(ctx);
		} else {
			match () {
				_ if self.config.wt_path.is_none() => {
					self.prompt_for_wt_path(ctx);
					STORE_CONF(&self.config);
				}
				_ if !self.config.blk_set => {
					self.prompt_for_config_blk(ctx);
					STORE_CONF(&self.config);
				}
				_ if !self.config.lang_folder_created => {
					self.prompt_for_lang_folder(ctx);
					STORE_CONF(&self.config);
				}
				_ if self.status_menu => {
					self.prompt_for_status(ctx);
				}
				_ if self.prompt_for_entry.show  => {
					self.prompt_for_entry(ctx);
					STORE_CONF(&self.config);
				}
				_ if self.prompt_for_backup.active => {
					self.prompt_for_backup(ctx);
					STORE_CONF(&self.config);
				}
				#[cfg(windows)]
				_ if !self.config.prompted_about_lang_perm => {
					self.prompt_lang_file_warn(ctx);
					STORE_CONF(&self.config);
				}
				_ => {}
			}
			self.render_header_bar(ctx, frame);
			CentralPanel::default().show(ctx, |ui| {
				ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
					ui.horizontal(|ui| {
						{
							if ui.add(Button::new("Add new entry")).clicked() {
								self.prompt_for_entry.show = true;
							}
						}

						{
							let lang_enabled = self.config.is_lang_enabled().unwrap_or(true);
							let lang_toggle_text: RichText = if lang_enabled {
								RichText::new("Global custom lang on").color(Color32::from_rgb(0, 255, 0))
							} else {
								RichText::new("Global custom lang off").color(Color32::from_rgb(255, 0, 0))
							};
							if ui.add(Button::new(lang_toggle_text)).clicked() {
								if let Some(path) = self.config.wt_path.as_ref() {
									let path = format!("{}/config.blk", path);
									match fs::read_to_string(&path) {
										Ok(file) => {
											const LOCALIZATION_TOGGLE: [&str; 2] = ["testLocalization:b=yes", "testLocalization:b=no"];
											let file = &file.replace(LOCALIZATION_TOGGLE[!lang_enabled as usize], LOCALIZATION_TOGGLE[lang_enabled as usize]);

											if fs::write(&path, file).is_ok() {
												if let Some(lang_enabled) = self.config.is_lang_enabled() {
													self.config.enable_lang = lang_enabled;
													STORE_CONF(&self.config);
												} else {
													self.prompt_error.err_value = Some("Failed to check if localization was defined in config.blk".to_owned());
													return;
												}
											}
										}
										Err(err) => {
											self.prompt_error.err_value = Some(format!("{}", err).to_owned());
											return;
										}
									}
								} else {
									self.prompt_error.err_value = Some("WT path should be set, but was none".to_owned());
									return;
								}
							}
						}

						{
							if ui.add(Button::new("Re-apply all lang changes")).clicked() {
								if let Some(path) = &self.config.wt_path.clone().as_ref() {
									let entries = READ_PRIMITIVE();

									PrimitiveEntry::replace_all_entries_direct_str(self, &entries, path, true);

									WRITE_PRIMITIVE(&entries);
								} else {
									self.prompt_error.err_value = Some("WT path should be set, but was none".to_owned());
									return;
								}
							}
						}

						{
							if ui.add(Button::new("Backups")).clicked() {
								self.prompt_for_backup.active = true;
							}
						}
					});

					ui.add_space(5.0);


					let prim_array = READ_PRIMITIVE();

					for (i, primitive_entry) in prim_array.iter().enumerate() {
						ui.horizontal(|ui| {
							ui.add(Label::new(RichText::new(format!("{} changed to {}", primitive_entry.original_english, primitive_entry.new_english))));
							if ui.add(Button::new(RichText::new("Undo").color(Color32::from_rgb(255, 0, 0)))).clicked() {
								self.undo_entry(i, primitive_entry);
								STORE_CONF(&self.config);
							}
						});
					}
				});
				render_footer(ctx);
			});
		}
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
		font_def.font_data.insert("RobotoMono".to_owned(), FontData::from_owned(include_bytes!("../../fonts/roboto_mono/static/RobotoMono-Medium.ttf").to_vec()));
		font_def.family_and_size.insert(Heading, (FontFamily::Proportional, 30.0));
		font_def.family_and_size.insert(Body, (FontFamily::Proportional, 20.0));
		font_def.fonts_for_family.get_mut(&Proportional).expect("Failed to set font definition").insert(0, "RobotoMono".to_owned());
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
			prompt_for_backup: PromptForBackup { active: false, backup_name: "".to_owned() },
			prompt_for_entry: PromptForEntry { show: false, before_after_entry: EMPTY_BEFORE_AFTER(), toggle_dropdown: LangType::default() },
			prompt_error: AppError { err_value: None },
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
					if ui.add(Button::new(RichText::new("🔄 Reset configuration").text_style(TextStyle::Body))).clicked() {
						if let Err(err) = confy::store(CONFIG_NAME, Configuration::default()) {
							self.prompt_error.err_value = Some(err.to_string());
							return;
						} else {
							frame.quit();
						}
					}

					if ui.add(Button::new(RichText::new("Status").text_style(TextStyle::Body))).clicked() {
						self.status_menu = !self.status_menu;
					}

					if ui.add(Button::new(if self.config.dark_mode { RichText::new("☀").text_style(TextStyle::Body) } else { RichText::new("🌙").text_style(TextStyle::Body) })).clicked() {
						self.config.dark_mode = !self.config.dark_mode;
						STORE_CONF(&self.config);
					}
				});
			});
			ui.add_space(10.);
		});
	}
}

fn render_footer(ctx: &CtxRef) {
	TopBottomPanel::bottom("footer").show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("Report bug", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/issues/new"));
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("© 2022 Warthunder-Open-Source-Foundation", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/LICENSE"));
		})
	});
}
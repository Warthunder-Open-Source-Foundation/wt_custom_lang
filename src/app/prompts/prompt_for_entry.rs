use std::{fs};

use eframe::egui::{Button, Color32, ComboBox, CtxRef, Hyperlink, RichText, TextEdit, TextStyle, Window};
use levenshtein::levenshtein;
use wt_csv::wtcsv::core::wtcsv::WTCSV;

use crate::{CustomLang};
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
use crate::local_storage::entries::{LANG_PATH, READ_PRIMITIVE, WRITE_PRIMITIVE};

pub const EMPTY_BEFORE_AFTER: fn() -> (String, String) = || {
	("".to_owned(), "".to_owned())
};

pub struct PromptForEntry {
	pub show: bool,
	pub before_after_entry: (String, String),
	pub toggle_dropdown: LangType,
	pub searchbar: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LangType {
	Units = 0,
	Ui = 1,
	CommonLanguages = 2,
	Menu = 3,
}

impl LangType {
	pub fn to_file_name(&self) -> &str {
		match self {
			LangType::Units => {
				"units"
			}
			LangType::Ui => {
				"ui"
			}
			LangType::CommonLanguages => {
				"_common_languages"
			}
			LangType::Menu => {
				"menu"
			}
		}
	}
}

impl Default for LangType {
	fn default() -> Self {
		LangType::Units
	}
}

impl CustomLang {
	pub fn prompt_for_entry(&mut self, ctx: &CtxRef) {
		if let Some(wt_raw) = self.config.wt_path.as_ref() {
			Window::new("Adding a new entry").show(ctx, |ui| {
				let mut original = self.prompt_for_entry.before_after_entry.clone();
				let path = format!("{}/lang/{}.csv", wt_raw, &self.prompt_for_entry.toggle_dropdown.to_file_name());

				let mut color = Color32::from_rgb(255, 255, 255);

				let incomplete = fs::read_to_string(&path).unwrap_or("".to_owned());
				let mut contains = || {
					if let Some(search) = &self.prompt_for_entry.searchbar {
						original.0 = search.clone();
						color = Color32::from_rgb(64, 64, 255);
					} else if incomplete.contains(&format!(r#""{}""#, &original.0)) {
						color = Color32::from_rgb(0, 255, 0);
					} else {
						color = Color32::from_rgb(255, 255, 255);
					}
				};

				ui.horizontal(|ui| {
					ComboBox::from_label("").selected_text(format!("{:?}", self.prompt_for_entry.toggle_dropdown)).show_ui(ui, |ui| {
						ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Units, "Unit");
						ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Ui, "Ui");
						ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::CommonLanguages, "Common language");
						ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Menu, "Menu");
					}, );
					ui.add(Hyperlink::from_label_and_url("What these options mean", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/wiki/Types-of-supported-language-files"));
				});


				contains();

				let search_icon = if self.prompt_for_entry.searchbar.is_some() {
					Button::new("❌")
				} else {
					Button::new("🔍")
				};


				ui.horizontal(|ui| {
					ui.add(TextEdit::singleline(&mut original.0).interactive(self.prompt_for_entry.searchbar.is_none()).hint_text("Old name").text_color(color));
					if ui.add(search_icon).clicked() {
						if self.prompt_for_entry.searchbar.is_none() {
							self.prompt_for_entry.searchbar = closest_word(&original.0, fs::read_to_string(&path).unwrap());
						} else {
							self.prompt_for_entry.searchbar = None;
						}
					}
				});
				ui.add(TextEdit::singleline(&mut original.1).hint_text("New name"));

				self.prompt_for_entry.before_after_entry = original;

				ui.horizontal(|ui| {
					if ui.add(Button::new(RichText::new("Create").text_style(TextStyle::Heading))).clicked() {
						let lang_type = self.prompt_for_entry.toggle_dropdown.to_file_name();
						let path: String = format!("{}/lang/{}.csv", wt_raw, lang_type);

						match fs::read_to_string(&path) {
							Ok(mut file) => {
								let entry = PrimitiveEntry {
									file: lang_type.to_owned(),
									id: None,
									original_english: self.prompt_for_entry.before_after_entry.0.trim().to_string(),
									new_english: self.prompt_for_entry.before_after_entry.1.trim().to_string(),
								};

								PrimitiveEntry::replace_all_entries_from_file_str(vec![entry.clone()], &mut file, true);

								if fs::write(&path, file).is_ok() {
									let mut old = READ_PRIMITIVE();

									old.push(entry);

									WRITE_PRIMITIVE(&old);
								}
								self.prompt_for_entry.before_after_entry = EMPTY_BEFORE_AFTER();
							}
							Err(error) => {
								self.prompt_error.err_value = Some(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
								return;
							}
						}
					}
					if ui.add(Button::new(RichText::new("Cancel").text_style(TextStyle::Heading))).clicked() {
						self.prompt_for_entry.show = false;
					}
				});
			});
		}
	}

	pub fn undo_entry(&mut self, i: usize, primitive_entry: &PrimitiveEntry) {
		if let Some(wt_raw) = self.config.wt_path.as_ref() {
			let lang_type = self.prompt_for_entry.toggle_dropdown.to_file_name();
			let path: String = format!("{}/lang/{}.csv", wt_raw, lang_type);

			match fs::read_to_string(&path) {
				Ok(mut file) => {
					let entry = PrimitiveEntry {
						file: primitive_entry.file.clone(),
						id: None,
						original_english: primitive_entry.new_english.clone(),
						new_english: primitive_entry.original_english.clone(),
					};

					PrimitiveEntry::replace_all_entries_from_file_str(vec![entry.clone()], &mut file, true);

					if fs::write(&path, file).is_ok() {
						match fs::read(&LANG_PATH.constructed_path) {
							Ok(entries) => {
								match serde_json::from_slice::<Vec<PrimitiveEntry>>(&entries) {
									Ok(mut old) => {
										old.remove(i);

										WRITE_PRIMITIVE(&old);
									}
									Err(error) => {
										self.prompt_error.err_value = Some(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
										return;
									}
								}
							}
							Err(error) => {
								self.prompt_error.err_value = Some(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
								return;
							}
						}
					}
				}
				Err(error) => {
					self.prompt_error.err_value = Some(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
					return;
				}
			}
		} else {
			self.prompt_error.err_value = Some("WT path should be set, but was none".to_owned());
			return;
		}
	}
}


pub fn closest_word(known: &str, file: String) -> Option<String> {
	let wtcsv = WTCSV::new_from_file(&file, "blank").unwrap();

	let mut vec: Vec<(usize, String)> = Vec::new();

	for record in wtcsv.records {
		let item = record.items[1].to_owned();
		if item.contains(known) {
			vec.push((levenshtein(&item, known), item));
		}
	}

	vec.sort_by_key(|x| x.0);

	if let Some(val) = vec.first() {
		if val.0 < known.len() {
			return Some(val.1.to_owned());
		}
	}
	None
}
use std::{fs};

use eframe::egui::{Button, Color32, ComboBox, CtxRef, Hyperlink, RichText, TextEdit, TextStyle, Window};

use crate::{CustomLang};
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
use crate::local_storage::entries::{LANG_PATH, READ_PRIMITIVE, WRITE_PRIMITIVE};

pub const EMPTY_BEFORE_AFTER: fn() -> (String, String) = ||{
	("".to_owned(), "".to_owned())
};

pub struct PromptForEntry {
	pub show: bool,
	pub before_after_entry: (String, String),
	pub toggle_dropdown: LangType,
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

				let mut color= Color32::from_rgb(255,255,255);;

				let mut contains = |file_path: &str|{
					if fs::read_to_string(format!("{}/lang/{}.csv", wt_raw, file_path)).unwrap_or("".to_owned()).contains(&format!(r#""{}""#, &original.0)) {
						color = Color32::from_rgb(0,255,0);

					} else {
						color = Color32::from_rgb(255,255,255);
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


				contains(self.prompt_for_entry.toggle_dropdown.to_file_name());

				ui.add(TextEdit::singleline (&mut original.0).hint_text("Old name").text_color(color));
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

			match  fs::read_to_string(&path) {
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
								match  serde_json::from_slice::<Vec<PrimitiveEntry>>(&entries) {
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
				},
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
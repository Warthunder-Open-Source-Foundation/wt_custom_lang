use std::{fs, thread};
use std::process::Command;

use eframe::egui::{Button, ComboBox, CtxRef, Hyperlink, RichText, TextEdit, TextStyle, Window};
use eframe::egui::Label;
use eframe::egui::style::Selection;
use execute::Execute;
use rfd::FileDialog;

use crate::CustomLang;
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;

pub struct PromptForEntry {
	pub add_csv_entry: Option<(String, String)>,
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
		Window::new("Adding a new entry").show(ctx, |ui| {
			let mut original = self.prompt_for_entry.add_csv_entry.clone().unwrap();
			ui.add(TextEdit::singleline(&mut original.0).hint_text("Old name"));
			ui.add(TextEdit::singleline(&mut original.1).hint_text("New name"));

			ui.horizontal(|ui| {
				ComboBox::from_label("").selected_text(format!("{:?}", self.prompt_for_entry.toggle_dropdown)).show_ui(ui, |ui| {
					ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Units, "Unit");
					ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Ui, "Ui");
					ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::CommonLanguages, "Common language");
					ui.selectable_value(&mut self.prompt_for_entry.toggle_dropdown, LangType::Menu, "Menu");
				}, );
				ui.add(Hyperlink::from_label_and_url("What these options mean", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/wiki/Types-of-supported-language-files"));
			});

			self.prompt_for_entry.add_csv_entry = Some(original);

			ui.horizontal(|ui| {
				if ui.add(Button::new(RichText::new("Create!").text_style(TextStyle::Heading))).clicked() {
					let lang_type = self.prompt_for_entry.toggle_dropdown.to_file_name();
					let path: String = format!("{}/lang/{}.csv", self.config.wt_path.as_ref().unwrap(), lang_type);

					let mut file = fs::read_to_string(&path).unwrap();

					let entry = PrimitiveEntry {
						file: lang_type.to_owned(),
						id: None,
						original_english: self.prompt_for_entry.add_csv_entry.as_ref().unwrap().0.trim().to_string(),
						new_english: self.prompt_for_entry.add_csv_entry.as_ref().unwrap().1.trim().to_string(),
					};

					PrimitiveEntry::replace_all_entries_from_file_str(vec![entry.clone()], &mut file, true);

					if fs::write(&path, file).is_ok() {
						let mut old: Vec<PrimitiveEntry> = serde_json::from_str(&self.config.primitive_entries).unwrap();
						old.push(entry);
						self.config.primitive_entries = serde_json::to_string(&old).unwrap();
					}
					self.prompt_for_entry.add_csv_entry = None;
				}
				if ui.add(Button::new(RichText::new("Cancel").text_style(TextStyle::Heading))).clicked() {
					self.prompt_for_entry.add_csv_entry = None;
				}
			});
		});
	}
	pub fn undo_entry(&mut self, i: usize, primitive_entry: &PrimitiveEntry) {
		let lang_type = self.prompt_for_entry.toggle_dropdown.to_file_name();
		let path: String = format!("{}/lang/{}.csv", self.config.wt_path.as_ref().unwrap(), lang_type);

		let mut file = fs::read_to_string(&path).unwrap();

		let entry = PrimitiveEntry {
			file: primitive_entry.file.clone(),
			id: None,
			original_english: primitive_entry.new_english.clone(),
			new_english: primitive_entry.original_english.clone(),
		};

		PrimitiveEntry::replace_all_entries_from_file_str(vec![entry.clone()], &mut file, true);

		if fs::write(&path, file).is_ok() {
			let mut old: Vec<PrimitiveEntry> = serde_json::from_str(&self.config.primitive_entries).unwrap();
			old.remove(i);
			self.config.primitive_entries = serde_json::to_string(&old).unwrap();
		}
	}
}
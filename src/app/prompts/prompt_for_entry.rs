use std::{fs, thread};
use std::process::Command;

use eframe::egui::*;
use eframe::egui::Button;
use eframe::egui::Label;
use eframe::egui::style::Selection;
use execute::Execute;
use rfd::FileDialog;
use crate::CustomLang;


use crate::lang_manipulation::primitive_lang::PrimitiveEntry;

pub struct PromptForEntry {
	pub add_csv_entry: Option<(String, String)>,
	pub toggle_dropdown: bool,
}

impl CustomLang {
	pub fn prompt_for_entry(&mut self, ctx: &CtxRef) {
		Window::new("Adding a new entry").show(ctx, |ui| {
			let mut original = self.prompt_for_entry.add_csv_entry.clone().unwrap();
			ui.add(TextEdit::singleline(&mut original.0).hint_text("Old name"));
			ui.add(TextEdit::singleline(&mut original.1).hint_text("New name"));

			#[derive(Debug, Eq, PartialEq)]
			enum LangType {
				Units = 0,
				Ui = 1,
				CommonLanguages = 2,
				Menu = 3,
			}

			let mut selected = LangType::CommonLanguages;

			ComboBox::from_label("Select one!").selected_text(format!("{:?}", selected)).show_ui(ui, |ui| {
				ui.selectable_value(&mut selected, LangType::Units, "Unit");
				ui.selectable_value(&mut selected, LangType::Ui, "Ui");
				ui.selectable_value(&mut selected, LangType::CommonLanguages, "Common language");
				ui.selectable_value(&mut selected, LangType::Menu, "Menu");
			},
			);

			self.prompt_for_entry.add_csv_entry = Some(original);

			ui.horizontal(|ui| {
				if ui.add(Button::new(RichText::new("Create!").text_style(TextStyle::Heading))).clicked() {
					let path = format!("{}/lang/units.csv", self.config.wt_path.as_ref().unwrap());
					let mut file = fs::read_to_string(&path).unwrap();

					let entry = PrimitiveEntry {
						id: None,
						original_english: self.prompt_for_entry.add_csv_entry.as_ref().unwrap().0.trim().to_string(),
						new_english: self.prompt_for_entry.add_csv_entry.as_ref().unwrap().1.trim().to_string(),
					};

					PrimitiveEntry::replace_all_entries(vec![entry.clone()], &mut file);

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
}
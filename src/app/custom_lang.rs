


use eframe::egui;
use eframe::egui::{Button, CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Layout, RichText, TextStyle, TopBottomPanel};
use eframe::egui::FontFamily::Proportional;

use eframe::egui::TextStyle::{Body, Heading};
use eframe::epi::{App, Frame, Storage};

use crate::app::prompts::prompt_for_entry::{EMPTY_BEFORE_AFTER, LangType, PromptForEntry};
use crate::config::Configuration;


use crate::{CONFIG_NAME, REPO_URL};
use crate::app::prompts::prompt_error::AppError;
use crate::app::prompts::prompt_for_backup::PromptForBackup;
use crate::app::update::update;
use crate::cache::cache::Cache;

pub struct CustomLang {
	pub config: Configuration,
	pub status_menu: bool,
	pub prompt_for_backup: PromptForBackup,
	pub prompt_for_entry: PromptForEntry,
	pub prompt_error: AppError,
	pub cache: Cache,
}

pub const STORE_CONF: fn(config: &Configuration) = |config| {
	if let Err(_err) = confy::store(CONFIG_NAME, config) {
		panic!("Failed to write to configuration file, this error will be discontinued soon");
	}
};

impl App for CustomLang {
	fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
		update(self, ctx, frame)
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
			prompt_for_entry: PromptForEntry { show: false, before_after_entry: EMPTY_BEFORE_AFTER(), toggle_dropdown: LangType::default(), searchbar: None },
			prompt_error: AppError { err_value: None },
			cache: Cache::new(),
		}
	}
	pub fn render_header_bar(&mut self, ctx: &CtxRef, frame: &Frame) {
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

pub fn render_footer(ctx: &CtxRef) {
	TopBottomPanel::bottom("footer").show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("Report bug", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/issues/new"));
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("© 2022 Warthunder-Open-Source-Foundation", "https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/LICENSE"));
		})
	});
}
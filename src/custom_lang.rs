use std::borrow::Cow;
use std::fs;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use eframe::egui::*;
use eframe::egui::Button;
use eframe::epi::{App, Frame, Storage};
use eframe::{egui, NativeOptions, run_native};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::Key::B;
use eframe::egui::TextStyle::{Body, Button as ButtonStyle, Heading};
use eframe::egui::Label;
use rfd::FileDialog;

use crate::config::Configuration;
use crate::REPO_URL;

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

pub struct CustomLang {
	pub config: Configuration,
	pub status_menu: bool,
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
			}
			_ if !self.config.blk_set => {
				self.prompt_for_config_blk(ctx);
			}
			_ if !self.config.lang_folder_created => {
				self.prompt_for_lang_folder(ctx);
			}
			_ if self.status_menu => {
				self.prompt_for_status(ctx);
			}
			_ => {}
		}
		self.render_header_bar(ctx, frame);
		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {});
			render_footer(ctx);
		});

		confy::store(CONFIG_NAME, &self.config).unwrap();
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
			ui.add(Hyperlink::from_label_and_url("Where the game might be installed",format!("{}/guide/install_folder.md", REPO_URL)));

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
		Window::new("Generating the lang folder").show(ctx, |ui| {
			ui.label(RichText::new("Launch the game and close it again"));
			if ui.add(Button::new("Check if it worked")).clicked() {
				if fs::read_dir(format!("{}/lang", self.config.wt_path.as_ref().unwrap())).is_ok() {
					self.config.lang_folder_created = true;
				}
			}
		});
	}
}

fn render_footer(ctx: &CtxRef) {
	TopBottomPanel::bottom("footer").show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(10.0);
			ui.add(Hyperlink::from_label_and_url("Report bug","https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/issues/new"));
			ui.add_space(10.0)
		})
	});
}
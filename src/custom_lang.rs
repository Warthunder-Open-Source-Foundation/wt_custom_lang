use std::borrow::Cow;
use std::fs;
use std::time::Duration;
use eframe::egui::{Button, CentralPanel, CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2, Visuals, Window};
use eframe::epi::{App, Frame, Storage};
use eframe::{egui, NativeOptions, run_native};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading};
use rfd::FileDialog;
use tracing_subscriber::fmt::format;
use crate::config::Configuration;
use crate::REPO_URL;

const CONFIG_NAME: &str = "wt_custom_lang"; //DO not change unless absolutely necessary

pub struct CustomLang {
	pub config: Configuration,
}

impl App for CustomLang {
	fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
		if self.config.dark_mode {
			ctx.set_visuals(Visuals::dark());
		} else {
			ctx.set_visuals(Visuals::light());
		}

		if self.config.wt_path.is_none() {
			self.prompt_for_wt_path(ctx);
		} else {
			self.render_header_bar(ctx, frame);
			CentralPanel::default().show(ctx, |ui| {
				render_header(ui);
				ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
					ui.label(r#"WIP"#);
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
	pub(crate) fn new() -> Self {
		let config: Configuration = confy::load(CONFIG_NAME).unwrap_or_default();
		Self {
			config,
		}
	}
	fn render_header_bar(&mut self, ctx: &CtxRef, frame: &Frame) {
		TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.add_space(10.);
			egui::menu::bar(ui, |ui| {
				ui.with_layout(Layout::left_to_right(), |ui| {
					ui.add(Hyperlink::new(format!("{}/guide/how_to_use.md", REPO_URL)).text("📓 How to use"));
				});
				ui.with_layout(Layout::right_to_left(), |ui| {
					// let close_btn = ui.add(Button::new("❌").text_style(TextStyle::Body));

					let refresh_btn = ui.add(Button::new("🔄 rest configuration").text_style(TextStyle::Body));

					if refresh_btn.clicked() {
						confy::store(CONFIG_NAME, Configuration::default()).unwrap();
						frame.quit();
					}

					let theme_btn = ui.add(Button::new(if self.config.dark_mode { "☀" } else { "🌙" }).text_style(TextStyle::Body));

					if theme_btn.clicked() {
						confy::store(CONFIG_NAME, &self.config).unwrap();
						self.config.dark_mode = !self.config.dark_mode;
					}
				});
			});
			ui.add_space(10.);
		});
	}
	fn prompt_for_wt_path(&mut self, ctx: &CtxRef) {
		Window::new("Select WarThunder location").show(ctx, |ui| {
			let select_button = ui.add(Button::new("Choose path").text_style(TextStyle::Body));
			ui.add(Hyperlink::new(format!("{}/guide/install_folder.md", REPO_URL)).text("Where the game might be installed"));


			if select_button.clicked() {
				if let Some(path) = FileDialog::new().pick_folder() {
					if fs::read(&format!("{}/launcher.exe", path.to_str().unwrap())).is_ok() {
						self.config.wt_path = Some(path.to_str().unwrap().to_owned());
						confy::store(CONFIG_NAME, &self.config).unwrap();
					} else {
						println!("{}", "Bad path");
					}
				}
			}
		});
	}
}

fn render_header(ui: &mut Ui) {}

fn render_footer(ctx: &CtxRef) {
	TopBottomPanel::bottom("footer").show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(10.0);
			ui.add(Hyperlink::new("https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/issues/new").text("Report bug"));
			ui.add_space(10.0)
		})
	});
}
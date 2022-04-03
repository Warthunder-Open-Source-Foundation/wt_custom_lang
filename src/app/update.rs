use std::{fs, thread};
use std::fs::File;
use std::io::Write;

use eframe::egui::{Button, CentralPanel, Color32, CtxRef, RichText, ScrollArea, Visuals};
use eframe::egui::Label;
use eframe::epi::Frame;
use reqwest::StatusCode;

use crate::{CustomLang, TRACKED_FILES};
use crate::app::custom_lang::{render_footer, STORE_CONF};
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
use crate::local_storage::entries::{READ_PRIMITIVE, WRITE_PRIMITIVE};

pub fn update(custom_lang: &mut CustomLang, ctx: &CtxRef, frame: &Frame) {
	if custom_lang.config.dark_mode {
		ctx.set_visuals(Visuals::dark());
	} else {
		ctx.set_visuals(Visuals::light());
	}
	if custom_lang.prompt_error.err_value.is_some() {
		custom_lang.prompt_error(ctx, frame);
	} else {
		match () {
			_ if custom_lang.config.wt_path.is_none() => {
				custom_lang.prompt_for_wt_path(ctx);
				STORE_CONF(&custom_lang.config);
			}
			_ if custom_lang.config.is_lang_enabled().is_none() => {
				custom_lang.prompt_for_config_blk(ctx);
				STORE_CONF(&custom_lang.config);
			}
			_ if !custom_lang.config.lang_folder_created => {
				custom_lang.prompt_for_lang_folder(ctx);
				STORE_CONF(&custom_lang.config);
			}
			_ if custom_lang.status_menu => {
				custom_lang.prompt_for_status(ctx);
			}
			_ if custom_lang.prompt_for_entry.show => {
				custom_lang.prompt_for_entry(ctx);
				STORE_CONF(&custom_lang.config);
			}
			_ if custom_lang.prompt_for_backup.active => {
				custom_lang.prompt_for_backup(ctx);
				STORE_CONF(&custom_lang.config);
			}
			#[cfg(windows)]
			_ if !custom_lang.config.prompted_about_lang_perm => {
				custom_lang.prompt_lang_file_warn(ctx);
				STORE_CONF(&custom_lang.config);
			}
			_ => {}
		}
		custom_lang.render_header_bar(ctx, frame);
		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
				ui.horizontal(|ui| {
					{
						if ui.add(Button::new("Add new entry")).clicked() {
							custom_lang.prompt_for_entry.show = true;
						}
					}

					{
						let lang_enabled = custom_lang.config.is_lang_enabled().unwrap_or(true);
						let lang_toggle_text: RichText = if lang_enabled {
							RichText::new("Global custom lang on").color(Color32::from_rgb(0, 255, 0))
						} else {
							RichText::new("Global custom lang off").color(Color32::from_rgb(255, 0, 0))
						};
						if ui.add(Button::new(lang_toggle_text)).clicked() {
							if let Some(path) = custom_lang.config.wt_path.as_ref() {
								let path = format!("{}/config.blk", path);
								match fs::read_to_string(&path) {
									Ok(file) => {
										const LOCALIZATION_TOGGLE: [&str; 2] = ["testLocalization:b=yes", "testLocalization:b=no"];
										let file = &file.replace(LOCALIZATION_TOGGLE[!lang_enabled as usize], LOCALIZATION_TOGGLE[lang_enabled as usize]);

										if fs::write(&path, file).is_ok() {
											if let Some(lang_enabled) = custom_lang.config.is_lang_enabled() {
												custom_lang.config.enable_lang = lang_enabled;
												STORE_CONF(&custom_lang.config);
											} else {
												custom_lang.prompt_error.err_value = Some("Failed to check if localization was defined in config.blk".to_owned());
												return;
											}
										}
									}
									Err(err) => {
										custom_lang.prompt_error.err_value = Some(format!("{:?} {}:{} {}", err, line!(), column!(), file!()));
										return;
									}
								}
							} else {
								custom_lang.prompt_error.err_value = Some("WT path should be set, but was none".to_owned());
								return;
							}
						}
					}

					{
						if ui.add(Button::new("Re-apply all lang changes")).clicked() {
							static SOURCE_URL: &str = "https://raw.githubusercontent.com/Warthunder-Open-Source-Foundation/wt_datamine_extractor/master/lang/";

							if let Some(wt_path) =  custom_lang.config.wt_path.clone() {
								let mut handles = vec![];
								for text in TRACKED_FILES {
									let cloned_path = wt_path.clone();
									let handle = thread::spawn(move || {
										if let Ok(res) = reqwest::blocking::get(format!("{SOURCE_URL}{}", text.clone())) {
											if res.status() == StatusCode::OK {
												if let Ok(bytes) = res.bytes() {
													if let Ok(file) = String::from_utf8(bytes.to_vec()) {
														let path = format!("{}/lang/{text}", cloned_path);
														match fs::write(path, file)  {
															Ok(_) => {}
															Err(err) => {
																panic!("Cant write {text} to file due to {err}");
															}
														}
													} else {
														panic!("Cant parse {text} to string");
													}
												} else {
													panic!("{text} has no bytes");
												}
											} else {
												panic!("{text} failed to request, status code was {}", res.status().as_u16());
											}
										} else {
											panic!("failed to request {text}");
										}
									});
									handles.push(handle);
								}

								for handle in handles {
									if handle.join().is_err() {
										custom_lang.prompt_error.err_value = Some("A thread failed to download and write new custom files".to_owned());
									}
								}


								// Start patching
								let entries = READ_PRIMITIVE();

								PrimitiveEntry::replace_all_entries_direct_str(custom_lang, &entries, &wt_path, true);

								if custom_lang.prompt_error.err_value.is_none() {
									WRITE_PRIMITIVE(&entries);
								}
							}else {
								custom_lang.prompt_error.err_value = Some("WT path should be set, but was none".to_owned());
								return;
							}
						}
					}

					{
						if ui.add(Button::new("Backups")).clicked() {
							custom_lang.prompt_for_backup.active = true;
						}
					}
				});

				ui.add_space(5.0);


				let prim_array = READ_PRIMITIVE();

				for (i, primitive_entry) in prim_array.iter().enumerate() {
					ui.horizontal(|ui| {
						ui.add(Label::new(RichText::new(format!("{} changed to {}", primitive_entry.original_english, primitive_entry.new_english))));
						if ui.add(Button::new(RichText::new("Undo").color(Color32::from_rgb(255, 0, 0)))).clicked() {
							custom_lang.undo_entry(i, primitive_entry);
							STORE_CONF(&custom_lang.config);
						}
					});
				}
			});
			render_footer(ctx);
		});
	}
}
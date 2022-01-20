use std::fs;
use std::ops::Deref;
use chrono::{NaiveDateTime, TimeZone};
use eframe::egui::{Button, Color32, CtxRef, Label, RichText, Window};
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use crate::{CustomLang, LANG_PATH};
use crate::local_storage::backup::{BACKUP_PATH, BACKUP_ROOT};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackupEntry {
	pub dest: String,
	pub date: i64,
}

pub const BACKUP_ENTRY_STORAGE: fn() -> String = ||{
	format!("{}/{}/{}/backups.json", &BACKUP_PATH.deref().base_path, &BACKUP_PATH.config.project_prefix,&BACKUP_PATH.config.sub_folder)
};

const COPY_OPTIONS: CopyOptions = CopyOptions {
	overwrite: true,
	skip_exist: false,
	buffer_size: 64000 ,
	copy_inside: false,
	content_only: true,
	depth: 0
};

fn create_backup(wt_folder: &str) {
	let bin = fs::read_to_string(&BACKUP_ENTRY_STORAGE()).unwrap();
	let mut backups: Vec<BackupEntry> = serde_json::from_str(&bin).unwrap();

	let time = chrono::Local::now();
	let path = format!("{}/{}/{}/backup_{}", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder, &time.timestamp().to_string());

	fs::create_dir_all(&path).unwrap();
	fs_extra::dir::copy(wt_folder, &path, &COPY_OPTIONS).unwrap();

	backups.push(BackupEntry {
		dest: path.clone(),
		date: time.timestamp(),
	});
	let bin = serde_json::to_string(&backups).unwrap();
	fs::write(&BACKUP_ENTRY_STORAGE(), bin).unwrap();
}

fn load_backup(wt_folder: &str, target_backup: &str) {
	fs_extra::dir::copy(&target_backup,wt_folder,&COPY_OPTIONS).unwrap();
}

impl CustomLang {
	pub fn prompt_for_backup(&mut self, ctx: &CtxRef) {
		Window::new("Manage backups").show(ctx, |ui| {
			ui.horizontal(|ui|{
				if ui.add(Button::new("Create backup")).clicked() {
					create_backup(&format!("{}/lang", &self.config.wt_path.as_ref().unwrap()));
				}

				if ui.add(Button::new("Close")).clicked() {
					self.prompt_for_backup = false;
				}
			});

			ui.add_space(15.0);

			let bin = fs::read_to_string(&BACKUP_ENTRY_STORAGE()).unwrap();
			let mut backups: Vec<BackupEntry> = serde_json::from_str(&bin).unwrap();

			for (i, backup) in backups.clone().iter().enumerate() {
				ui.horizontal(|ui|{
					let time = NaiveDateTime::from_timestamp(backup.date, 0).to_string();
					ui.add(Label::new(format!("Created: {:?}", time)));
					if ui.add(Button::new("Load")).clicked() {
						load_backup(&format!("{}/lang", &self.config.wt_path.as_ref().unwrap()), &backup.dest);
					}

					if ui.add(Button::new(RichText::new("Delete backup").color(Color32::from_rgb(255, 0, 0)))).clicked() {
						backups.remove(i);
					}
				});
			}

			let bin = serde_json::to_string(&backups).unwrap();
			fs::write(&BACKUP_ENTRY_STORAGE(), bin).unwrap();
		});
	}
}
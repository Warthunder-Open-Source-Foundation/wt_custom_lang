use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use eframe::egui::{Button, CtxRef, Label, Window};
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use crate::{CustomLang, LANG_PATH};
use crate::local_storage::backup::{BACKUP_PATH, BACKUP_ROOT};

pub const BACKUPS_STORAGE: fn() -> String = ||{
	format!("{}/backups.json", &BACKUP_ROOT.deref())
};

const COPY_OPTIONS: CopyOptions = CopyOptions {
	overwrite: true,
	skip_exist: false,
	buffer_size: 64000 ,
	copy_inside: false,
	content_only: true,
	depth: 0
};

fn store_current_backup(wt_folder: &str) {
	let time = chrono::Local::now();
	let path = format!("{}/{}/{}/backup_{}", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder, &time.timestamp().to_string());

	fs::create_dir_all(&path).unwrap();
	fs_extra::dir::copy(wt_folder, &path, &COPY_OPTIONS).unwrap();
}

impl CustomLang {
	pub fn prompt_for_backup(&mut self, ctx: &CtxRef) {
		Window::new("Manage backups").show(ctx, |ui| {
			let paths = fs::read_dir(&BACKUP_ROOT.deref()).unwrap();
			for path in paths {
				println!("Name: {}", path.unwrap().path().display())
			}


			if ui.add(Button::new("Create backup")).clicked() {
				store_current_backup(&format!("{}", &BACKUP_ROOT.deref()));
			}

			ui.add_space(15.0);


			// for backup in &backups {
			// 	ui.add(Label::new(format!("{}", backup)));
			// 	ui.add_space(5.0);
			// }
		});
	}
}
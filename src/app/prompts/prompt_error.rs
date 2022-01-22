use std::fs;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use eframe::egui::{CtxRef, Label, Ui, Window};
use notify_rust::Notification;
use crate::{CONFIG_NAME, CustomLang};

pub struct AppError {
	pub err_value: Option<String>,
}

impl CustomLang {
	pub fn prompt_error(&self, ctx: &CtxRef) {
		Window::new("An error occurred").show(ctx, |ui|{
			if let Some(error) = self.prompt_error.err_value.as_ref() {
				println!("{}", error);
				store_err(&error);
				ui.add(Label::new(&**error));
				sleep(Duration::from_secs(10));
				exit(1);
			} else {
				// No but seriously, this function should only be called if err_value is some in the first place
				panic!("{}", "This level of failure should be impossible to reach");
			}
		});
	}
}

pub fn store_err(error: &str) {
	let err_notification = || {
		// Error dropped as there is quite literally nothing that can be done at this point
		let _ = Notification::new()
			.summary("WT-custom-lang exited unexpectedly")
			.body("if this issue keeps occurring please open an issue")
			.show();
	};

	if let Some(dir) = directories::BaseDirs::new() {
		if let Some(data_dir) = dir.data_dir().to_str() {
			let final_path = &format!("{}/{}/error/{}.log", data_dir, CONFIG_NAME, chrono::offset::Local::now().format("%Y-%m-%d--%H-%M-%S"));

			let _ = fs::create_dir_all(&format!("{}/{}/error", data_dir, CONFIG_NAME));

			match fs::write(final_path, error) {
				Ok(_) => {
					println!("Error log written to {}", final_path);
				}
				Err(err) => {
					println!("Failed to save error log due to:  {}", err);
				}
			}
		} else {
			err_notification();
		}
	} else {
		err_notification();
	}
}
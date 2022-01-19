use lazy_static::lazy_static;
use duckstore::{DirType, PathConfig, ResolvedPaths};

use crate::{CONFIG_NAME};

pub const BACKUP_CONFIG: PathConfig = PathConfig {
	project_prefix: CONFIG_NAME,
	sub_folder: "backup",
	file_name: "placeholder",
	dir_type: &DirType::Data,
};

lazy_static! {
 pub static ref BACKUP_PATH: ResolvedPaths<'static> = {
		match BACKUP_CONFIG.resolve() {
			Ok(x) => x,
			// Unsafe unwrap as this error cannot be recovered at runtime
			Err(err) => panic!("{err}"),
		}
    };
	pub static ref BACKUP_ROOT: String = format!("{}/{}/{}", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder);
}
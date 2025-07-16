use std::sync::LazyLock;
use std::{env, path::PathBuf};

use crate::config::parser::parser_config;
use crate::constant::config::AppConfig;

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(&env::current_exe().unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
});
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| CURRENT_DIR.join("config.json"));
pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| parser_config().unwrap());

pub mod config;
pub mod connection;
pub mod emulator;

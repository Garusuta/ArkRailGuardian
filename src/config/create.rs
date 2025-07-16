use crate::constant::CONFIG_PATH;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "configs/"]
struct Configs;

pub fn create_config() {
    if !CONFIG_PATH.exists() {
        Configs::get("config.json")
            .map(|file| {
                std::fs::write(&*CONFIG_PATH, file.data).expect("Failed to write config file");
                println!("Config file created at: {}", CONFIG_PATH.display());
            })
            .unwrap_or_else(|| {
                eprintln!("Failed to find the embedded config file.");
            });
    }
}

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use log::{info};

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub last_archive_path: Option<PathBuf>,
    pub font_size: f32,
    pub is_dark_mode: Option<bool>,
}

impl AppConfig {
    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir()
            .map(|dir| dir.join("nisabo/config.toml"))
    }

    pub fn load_config() -> Self {
        info!("loading config");
        if let Some(config_path) = Self::get_config_path() {
            info!("{:?}", config_path);

            if let Ok(data) = fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str::<AppConfig>(&data) {
                    return config;
                }
            }
        }
        AppConfig::default()
    }

    pub fn save_config(&self) {
        info!("saving config");
        if let Some(config_path) = Self::get_config_path() {
            info!("{:?}", config_path);
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            let config = AppConfig {
                last_archive_path: self.last_archive_path.clone(),
                font_size: self.font_size,
                is_dark_mode: self.is_dark_mode,
            };
            println!("Values: {}", self.font_size);

            if let Ok(toml_str) = toml::to_string_pretty(&config) {
                let _ = fs::File::create(&config_path)
                    .and_then(|mut f| f.write_all(toml_str.as_bytes()));
            }
        }
    }
}

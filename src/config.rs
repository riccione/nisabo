use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub last_archive_path: Option<PathBuf>,
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
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            /*
            let config = AppConfig {
                last_archive_path: self.archive_path.clone(),
            };
            */

            if let Ok(toml_str) = toml::to_string_pretty(&self) {
                let _ = fs::File::create(&config_path)
                    .and_then(|mut f| f.write_all(toml_str.as_bytes()));
            }
        }
    }
}

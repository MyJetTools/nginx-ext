use serde::{Deserialize, Serialize};

use crate::{config_path::ConfigPath, storage::nginx::instance::NginxPath};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "StartNginx")]
    pub start_nginx: bool,
    #[serde(rename = "NginxPath")]
    pub nginx_path: String,
    #[serde(rename = "ConfigPath")]
    pub config_path: String,
}

impl SettingsReader {
    pub async fn get_start_nginx(&self) -> bool {
        let read_access = self.settings.read().await;
        read_access.start_nginx
    }
    pub async fn get_config_path(&self) -> ConfigPath {
        let read_access = self.settings.read().await;
        ConfigPath::new(format_path(&read_access.config_path.as_str()))
    }

    pub async fn get_nginx_path(&self) -> NginxPath {
        let read_access = self.settings.read().await;
        NginxPath::new(format_path(&read_access.nginx_path.as_str()))
    }
}

fn format_path(src: &str) -> String {
    let mut result = if src.starts_with("~") {
        let home = std::env::var("HOME").unwrap();
        src.replace("~", home.as_str())
    } else {
        src.to_string()
    };

    if !result.ends_with(std::path::MAIN_SEPARATOR) {
        result.push(std::path::MAIN_SEPARATOR);
    }

    result
}

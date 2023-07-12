use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "DataPath")]
    pub data_path: String,
    #[serde(rename = "StartNginx")]
    pub start_nginx: bool,
    #[serde(rename = "NginxConfigFileName")]
    pub nginx_config_file_name: String,
}

impl SettingsReader {
    pub async fn get_start_nginx(&self) -> bool {
        let read_access = self.settings.read().await;
        read_access.start_nginx
    }
    pub async fn get_data_path(&self) -> String {
        let read_access = self.settings.read().await;
        format_path(read_access.data_path.as_str())
    }

    pub async fn get_nginx_config_file_name(&self) -> String {
        let read_access = self.settings.read().await;
        format_file(read_access.nginx_config_file_name.as_str())
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

fn format_file(src: &str) -> String {
    let mut result = if src.starts_with("~") {
        let home = std::env::var("HOME").unwrap();
        src.replace("~", home.as_str())
    } else {
        src.to_string()
    };

    result
}

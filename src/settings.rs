use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "DataPath")]
    pub data_path: String,
}

impl SettingsReader {
    pub async fn get_data_path(&self) -> String {
        let read_access = self.settings.read().await;
        format_path(read_access.data_path.as_str())
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

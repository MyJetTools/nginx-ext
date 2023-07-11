use my_no_sql_data_writer::MyNoSqlWriterSettings;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "DataPath")]
    pub data_path: String,

    #[serde(rename = "MyNoSqlDataWriterUrl")]
    pub my_no_sql_data_writer_url: String,
}

impl SettingsReader {
    pub async fn get_data_path(&self) -> String {
        let read_access = self.settings.read().await;
        format_path(read_access.data_path.as_str())
    }
}

#[async_trait::async_trait]
impl MyNoSqlWriterSettings for SettingsReader {
    async fn get_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_no_sql_data_writer_url.clone()
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

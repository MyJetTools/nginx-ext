use my_no_sql_data_writer::MyNoSqlWriterSettings;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "RevokedCrlPemFileFullPath")]
    pub revoked_crl_pem_file: String,

    #[serde(rename = "MyNoSqlDataWriterUrl")]
    pub my_no_sql_data_writer_url: String,
}

impl SettingsReader {
    pub async fn get_revoked_crl_pem_file(&self) -> String {
        let read_access = self.settings.read().await;
        format_path(read_access.revoked_crl_pem_file.as_str())
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
    if src.starts_with("~") {
        let home = std::env::var("HOME").unwrap();
        return src.replace("~", home.as_str());
    }

    src.to_string()
}

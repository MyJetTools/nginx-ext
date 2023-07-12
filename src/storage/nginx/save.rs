use crate::settings::SettingsReader;

use super::models::NginxFileContent;

pub async fn save(settings_reader: &SettingsReader, content: &NginxFileContent) {
    let file = super::get_nginx_file_name(settings_reader).await;
    let content: Vec<u8> = serde_json::to_vec_pretty(content).unwrap();
    tokio::fs::write(file, content.as_slice()).await.unwrap();
}

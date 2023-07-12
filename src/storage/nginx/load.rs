use crate::settings::SettingsReader;

use super::models::NginxFileContent;

pub async fn load(settings_reader: &SettingsReader) -> NginxFileContent {
    let file = super::get_nginx_file_name(settings_reader).await;

    let content = tokio::fs::read_to_string(file).await;

    if let Err(err) = &content {
        println!(
            "Can not load nginx configuration. Creating new one. Err: {}",
            err
        );

        return NginxFileContent::default();
    }

    let content = content.unwrap();

    serde_yaml::from_str(content.as_str()).unwrap()
}

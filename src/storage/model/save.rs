use crate::{settings::SettingsReader, storage::model::ConfigFileContent};

pub async fn save(settings_reader: &SettingsReader, content: &ConfigFileContent) {
    let ca_path = settings_reader.get_config_path().await;
    let file_name = ca_path.into_nginx_yaml_config_file_name();

    let content = serde_yaml::to_string(content).unwrap();
    tokio::fs::write(file_name, content.as_bytes())
        .await
        .unwrap();
}

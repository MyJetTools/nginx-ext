use crate::{settings::SettingsReader, storage::model::ConfigFileContent};

pub async fn load(settings_reader: &SettingsReader) -> ConfigFileContent {
    let ca_path = settings_reader.get_config_path().await;

    let file_name = ca_path.into_nginx_yaml_config_file_name();

    let content = tokio::fs::read_to_string(file_name).await;

    if let Err(err) = &content {
        println!(
            "Can not load nginx configuration. Creating new one. Err: {}",
            err
        );

        return ConfigFileContent::default();
    }

    let content = content.unwrap();

    serde_yaml::from_str(content.as_str()).unwrap()
}

use crate::{app::AppContext, storage::nginx::models::NginxFileContent};

pub async fn generate_config_file(app: &AppContext, content: &NginxFileContent) {
    let file = app.settings_reader.get_nginx_config_file_name().await;

    let mut result = String::new();
    content.generate_nginx_up_streams_configuration(&mut result);
    content.generate_nginx_http_configuration(&mut result);

    tokio::fs::write(file, result).await.unwrap();
}

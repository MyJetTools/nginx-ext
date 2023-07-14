use crate::app::AppContext;

pub async fn delete(app: &AppContext, domain: &str) -> bool {
    let mut nginx_content = app.nginx_file_content.write().await;

    if let Some(http_configs) = &mut nginx_content.http_configs {
        let result: bool = http_configs.remove(domain).is_some();

        if result {
            super::super::save(&app.settings_reader, &nginx_content).await;
            crate::flows::generate_nginx_config_and_reload_nginx(&app, &nginx_content).await;
        }
        return result;
    }

    false
}

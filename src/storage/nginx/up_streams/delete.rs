use crate::app::AppContext;

pub async fn delete(app: &AppContext, name: &str) -> bool {
    let mut nginx_content = app.nginx_file_content.write().await;

    if let Some(upstreams) = &mut nginx_content.upstreams {
        let result: bool = upstreams.remove(name).is_some();

        if result {
            super::super::save(&app.settings_reader, &nginx_content).await;
            super::super::instance::generate_config_file(&app, &nginx_content).await;
        }
        return result;
    }

    false
}

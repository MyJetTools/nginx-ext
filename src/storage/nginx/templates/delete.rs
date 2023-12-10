use crate::app::AppContext;

pub async fn delete(app: &AppContext, id: &str) -> bool {
    let mut nginx_content = app.config_file_content.write().await;

    if let Some(templates) = &mut nginx_content.templates {
        let result: bool = templates.remove(id).is_some();

        if result {
            crate::storage::model::save(&app.settings_reader, &nginx_content).await;
            crate::flows::generate_nginx_config_and_reload_nginx(&app, &nginx_content)
                .await
                .unwrap();
        }
        return result;
    }

    false
}

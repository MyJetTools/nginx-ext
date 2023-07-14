use crate::app::AppContext;

pub async fn insert_or_replace(app: &AppContext, id: String, lines: Vec<String>) {
    let mut nginx_content = app.nginx_file_content.write().await;

    if nginx_content.templates.is_none() {
        nginx_content.templates = Some(Default::default());
    }

    let templates = nginx_content.templates.as_mut().unwrap();

    templates.insert(id, lines);
    super::super::save(&app.settings_reader, &nginx_content).await;

    crate::flows::generate_nginx_config_and_reload_nginx(&app, &nginx_content).await;
}

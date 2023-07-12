use crate::app::AppContext;

pub async fn delete(app: &AppContext, name: &str) -> bool {
    let mut nginx_content = app.nginx_file_content.write().await;
    let result = nginx_content.upstreams.remove(name).is_some();

    if result {
        super::super::save(&app.settings_reader, &nginx_content).await;
        super::super::instance::generate_up_streams_file(&nginx_content).await;
    }

    result
}

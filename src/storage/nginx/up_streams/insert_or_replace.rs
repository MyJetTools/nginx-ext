use crate::app::AppContext;

use super::super::models::UpStreamRouteStorageModel;

pub async fn insert_or_replace(
    app: &AppContext,
    name: String,
    routes: Vec<UpStreamRouteStorageModel>,
) {
    let mut nginx_content = app.nginx_file_content.write().await;

    if nginx_content.upstreams.is_none() {
        nginx_content.upstreams = Some(Default::default());
    }

    let upstreams = nginx_content.upstreams.as_mut().unwrap();

    upstreams.insert(name, routes);
    super::super::save(&app.settings_reader, &nginx_content).await;
    crate::flows::generate_nginx_config_and_reload_nginx(&app, &nginx_content).await;
}

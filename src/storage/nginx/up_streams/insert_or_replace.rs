use crate::app::AppContext;

use super::super::models::UpStreamRouteStorageModel;

pub async fn insert_or_replace(
    app: &AppContext,
    name: String,
    routes: Vec<UpStreamRouteStorageModel>,
) {
    let mut nginx_content = app.nginx_file_content.write().await;
    nginx_content.upstreams.insert(name.to_string(), routes);
    super::super::save(&app.settings_reader, &nginx_content).await;
    super::super::instance::generate_up_streams_file(&nginx_content).await;
}

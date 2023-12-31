use crate::{app::AppContext, storage::model::HttpConfig};

pub async fn insert_or_replace(app: &AppContext, domain: String, config: HttpConfig) {
    let mut nginx_content = app.config_file_content.write().await;

    if nginx_content.http_configs.is_none() {
        nginx_content.http_configs = Some(Default::default());
    }

    let http_configs = nginx_content.http_configs.as_mut().unwrap();
    http_configs.insert(domain, config);
    crate::storage::model::save(&app.settings_reader, &nginx_content).await;
}

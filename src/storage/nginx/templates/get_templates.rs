use std::collections::BTreeMap;

use crate::app::AppContext;

pub async fn get_templates(app: &AppContext) -> Option<BTreeMap<String, Vec<String>>> {
    let nginx_content = app.config_file_content.read().await;
    nginx_content.templates.clone()
}

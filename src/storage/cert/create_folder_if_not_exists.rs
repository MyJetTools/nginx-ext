use crate::app::AppContext;

pub async fn create_folder_if_not_exists(app: &AppContext, ca_cn: &str, email: &str) -> String {
    let path = super::compile_cert_path(app, ca_cn, email).await;

    let _ = tokio::fs::create_dir_all(path.as_str()).await;

    path
}

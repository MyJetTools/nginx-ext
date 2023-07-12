use crate::app::AppContext;

use super::CertPath;

pub async fn create_folder_if_not_exists(app: &AppContext, ca_cn: &str, email: &str) -> CertPath {
    let path = CertPath::new(app, ca_cn, email).await;

    let _ = tokio::fs::create_dir_all(path.as_str()).await;

    path
}

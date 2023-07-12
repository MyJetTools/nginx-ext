use crate::{app::AppContext, pem::PemPrivateKey};

use super::CaPath;

pub async fn load_private_key(app: &AppContext, ca_cn: &str) -> PemPrivateKey {
    let file_name = CaPath::new(app, ca_cn).await.into_private_key_file_name();
    let content = tokio::fs::read(file_name.as_str()).await.unwrap();
    PemPrivateKey::from_bytes(content)
}

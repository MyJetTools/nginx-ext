use crate::{app::AppContext, pem::PemPrivateKey};

use super::ClientCertPath;

pub async fn load_pem_private_key(app: &AppContext, ca_cn: &str, email: &str) -> PemPrivateKey {
    let cert_path = ClientCertPath::new(app, ca_cn, email)
        .await
        .into_private_key_file_name();

    let content = tokio::fs::read(cert_path).await.unwrap();

    PemPrivateKey::from_bytes(content)
}

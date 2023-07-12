use crate::{app::AppContext, pem::PemCertificate};

use super::CertPath;

pub async fn load_pem_certificate(app: &AppContext, ca_cn: &str, email: &str) -> PemCertificate {
    let cert_path = CertPath::new(app, ca_cn, email).await;

    let content = tokio::fs::read(cert_path.to_cert_file_name())
        .await
        .unwrap();
    PemCertificate::from_bytes(content)
}

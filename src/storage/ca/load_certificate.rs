use crate::{app::AppContext, pem::PemCertificate};

use super::CaPath;

pub async fn load_certificate(app: &AppContext, ca_cn: &str) -> PemCertificate {
    let file_name = CaPath::new(app, ca_cn).await.into_cert_file_name();
    let content = tokio::fs::read(file_name.as_str()).await.unwrap();
    PemCertificate::from_bytes(content)
}

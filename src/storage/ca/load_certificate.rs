use crate::{app::AppContext, pem::PemCertificate};

pub async fn load_certificate(app: &AppContext, ca_cn: &str) -> PemCertificate {
    let file_name = app
        .settings_reader
        .get_ca_data_path(ca_cn.into())
        .await
        .into_cert_file_name();
    let content = tokio::fs::read(file_name.as_str()).await.unwrap();
    PemCertificate::from_bytes(content)
}

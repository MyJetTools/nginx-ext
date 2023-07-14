use crate::{app::AppContext, pem::PemCertificate};

pub async fn load_certificate(app: &AppContext, ca_cn: &str) -> PemCertificate {
    let ca_path = app
        .settings_reader
        .get_config_path()
        .await
        .into_ca_data_path(ca_cn);

    let file_name = ca_path.into_cert_file_name();

    let content = tokio::fs::read(file_name.as_str()).await.unwrap();
    PemCertificate::from_bytes(content)
}

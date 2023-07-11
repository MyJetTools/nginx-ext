use crate::app::AppContext;

pub async fn write(
    app: &AppContext,
    ca_cn: &str,
    email: &str,
    public_key_pem: Vec<u8>,
    private_key_pem: Vec<u8>,
    cert_pem: Vec<u8>,
) {
    let path = super::compile_cert_path(app, ca_cn, email).await;

    tokio::fs::write(format!("{}/public_key.pem", path), public_key_pem)
        .await
        .unwrap();

    tokio::fs::write(format!("{}/private_key.pem", path), private_key_pem)
        .await
        .unwrap();

    tokio::fs::write(format!("{}/cert.pem", path), cert_pem)
        .await
        .unwrap();
}

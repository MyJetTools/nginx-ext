use crate::app::AppContext;

pub async fn load_private_key(
    app: &AppContext,
    ca_cn: &str,
) -> openssl::pkey::PKey<openssl::pkey::Private> {
    let file_name = super::super::utils::get_ca_private_key_file_name(app, ca_cn).await;

    let private_key_pem = tokio::fs::read_to_string(file_name.as_str()).await.unwrap();

    openssl::pkey::PKey::private_key_from_pem(private_key_pem.as_bytes()).unwrap()
}

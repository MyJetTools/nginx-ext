use crate::{app::AppContext, pem::PemPrivateKey};

pub async fn load_private_key(app: &AppContext, ca_cn: &str) -> PemPrivateKey {
    let ca_path = app
        .settings_reader
        .get_config_path()
        .await
        .into_ca_data_path(ca_cn);

    let file_name = ca_path.into_private_key_file_name();

    println!("Loading private key from file: {}", file_name);
    let content = tokio::fs::read(file_name.as_str()).await.unwrap();
    PemPrivateKey::from_bytes(content)
}

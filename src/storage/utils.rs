use crate::app::AppContext;

pub const CA_CERT_FILE_NAME: &str = "ca_cert.pem";
pub const CA_PUBLIC_KEY_FILE_NAME: &str = "ca_public_key.pem";
pub const CA_PRIVATE_KEY_FILE_NAME: &str = "ca_private_key.pem";

pub const CERT_FILE_NAME: &str = "cert.pem";
pub const PUBLIC_KEY_FILE_NAME: &str = "public_key.pem";
pub const PRIVATE_KEY_FILE_NAME: &str = "private_key.pem";

pub const OPEN_SSL_CNF: &str = "openssl.cnf";

pub const SERIAL_FILE_NAME: &str = "serial";

pub const CRL_FILE_NAME: &str = "crl.pem";

pub async fn get_ca_path(app: &AppContext, ca_cn: &str) -> String {
    let mut path = app.settings_reader.get_data_path().await;
    path.push_str(ca_cn);
    path
}

pub async fn get_ca_private_key_file_name(app: &AppContext, ca_cn: &str) -> String {
    let mut path = get_ca_path(app, ca_cn).await;
    path.push('/');
    path.push_str(CA_PRIVATE_KEY_FILE_NAME);
    path
}

pub async fn get_ca_cert_file_name(app: &AppContext, ca_cn: &str) -> String {
    let mut path = get_ca_path(app, ca_cn).await;
    path.push('/');
    path.push_str(CA_CERT_FILE_NAME);
    path
}

pub async fn get_config_name(app: &AppContext, ca_cn: &str) -> String {
    let mut path = get_ca_path(app, ca_cn).await;
    path.push('/');
    path.push_str(OPEN_SSL_CNF);
    path
}

pub async fn get_serial_file_name(app: &AppContext, ca_cn: &str) -> String {
    let mut path = get_ca_path(app, ca_cn).await;
    path.push('/');
    path.push_str(SERIAL_FILE_NAME);
    path
}

pub async fn get_crl_pem_file_name(app: &AppContext, ca_cn: &str) -> String {
    let mut path = get_ca_path(app, ca_cn).await;
    path.push('/');
    path.push_str(CRL_FILE_NAME);
    path
}

use openssl::x509::X509;

use super::super::utils::*;
use crate::app::AppContext;

pub async fn write(
    app: &AppContext,
    ca_cn: &str,
    country_code: &str,
    organization_name: &str,
    city: &str,
    cert_ca: X509,
    public_key: Vec<u8>,
    private_key: Vec<u8>,
) {
    let path = super::super::utils::get_ca_path(app, ca_cn).await;
    tokio::fs::create_dir(path.as_str()).await.unwrap();

    let serial_file = format!("{}/serial", path);
    tokio::fs::write(serial_file.as_str(), "00").await.unwrap();

    let index_file_name = format!("{}/index.txt", path);
    tokio::fs::write(index_file_name.as_str(), "")
        .await
        .unwrap();

    let ca_cert_file_name = format!("{}/{}", path, CA_CERT_FILE_NAME);
    tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.to_pem().unwrap())
        .await
        .unwrap();

    let ca_private_key_file_name = format!("{}/{}", path, CA_PRIVATE_KEY_FILE_NAME);
    tokio::fs::write(ca_private_key_file_name.as_str(), private_key)
        .await
        .unwrap();

    let serial_file = format!("{}/openssl.cnf", path);
    tokio::fs::write(
        serial_file.as_str(),
        format!(
            r#"
[ ca ]
default_ca = {ca_cn}
            
[ {ca_cn} ]
dir = {path}
database = {index_file_name}
certificate = {ca_cert_file_name}
private_key = {ca_private_key_file_name}
serial = {serial_file}
default_days = 3650
default_md = sha256
policy = policy_any
default_crl_days = 3650
            
[ policy_any ]
countryName = {country_code}
organizationName = {organization_name}
localityName = {city}
"#
        ),
    )
    .await
    .unwrap();

    tokio::fs::write(format!("{}/{}", path, CA_PUBLIC_KEY_FILE_NAME), public_key)
        .await
        .unwrap();

    tokio::fs::write(format!("{}/index.txt.attr", path), "unique_subject = yes")
        .await
        .unwrap();
}

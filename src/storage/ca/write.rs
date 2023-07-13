use openssl::x509::X509;

use crate::{app::AppContext, storage::nginx::instance::SslCertsPath};

use super::CaPath;

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
    let ca_path = CaPath::new(app, ca_cn).await;
    tokio::fs::create_dir(ca_path.as_str()).await.unwrap();

    let serial_file_name = ca_path.to_serial_file_name();
    tokio::fs::write(serial_file_name.as_str(), "00")
        .await
        .unwrap();

    let index_file_name = ca_path.to_index_file_name();
    tokio::fs::write(index_file_name.as_str(), "")
        .await
        .unwrap();

    let ca_cert_file_name = ca_path.to_cert_file_name();

    let cert_ca = cert_ca.to_pem().unwrap();
    tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
        .await
        .unwrap();

    let ssl_path = SslCertsPath::new(&app.settings_reader).await;

    tokio::fs::write(ssl_path.generate_ca_cert_file(ca_cn), cert_ca.as_slice())
        .await
        .unwrap();

    tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
        .await
        .unwrap();

    let ca_private_key_file_name = ca_path.to_private_key_file_name();
    tokio::fs::write(ca_private_key_file_name.as_str(), private_key)
        .await
        .unwrap();

    tokio::fs::write(
        ca_path.to_config_file_name(),
        format!(
            r#"
[ ca ]
default_ca = {ca_cn}
            
[ {ca_cn} ]
dir = {path}
database = {index_file_name}
certificate = {ca_cert_file_name}
private_key = {ca_private_key_file_name}
serial = {serial_file_name}
default_days = 3650
default_md = sha256
policy = policy_any
default_crl_days = 3650
            
[ policy_any ]
countryName = {country_code}
organizationName = {organization_name}
localityName = {city}
"#,
            path = ca_path.as_str(),
        ),
    )
    .await
    .unwrap();

    tokio::fs::write(ca_path.to_public_key_file_name(), public_key)
        .await
        .unwrap();

    tokio::fs::write(ca_path.to_index_attr_file_name(), "unique_subject = yes")
        .await
        .unwrap();
}

use crate::{
    app::AppContext,
    pem::{PemCertInfo, PemCertificate, PemPrivateKey},
};

pub async fn write(
    app: &AppContext,
    cert_info: &PemCertInfo,
    cert_ca: PemCertificate,
    private_key: PemPrivateKey,
) {
    let ca_path = app
        .settings_reader
        .get_config_path()
        .await
        .into_ca_data_path(&cert_info.ca_cn);

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

    tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
        .await
        .unwrap();

    //let ssl_path = SslCertsPath::new(&app.settings_reader).await;
    let nginx_path = app.settings_reader.get_nginx_path().await;

    tokio::fs::write(
        nginx_path.get_ca_cert_file(&cert_info.ca_cn),
        cert_ca.as_slice(),
    )
    .await
    .unwrap();

    tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
        .await
        .unwrap();

    let ca_private_key_file_name = ca_path.to_private_key_file_name();
    tokio::fs::write(ca_private_key_file_name.as_str(), private_key.as_slice())
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
            ca_cn = cert_info.ca_cn,
            country_code = cert_info.country_code,
            organization_name = cert_info.organization,
            city = cert_info.city,
        ),
    )
    .await
    .unwrap();

    tokio::fs::write(ca_path.to_index_attr_file_name(), "unique_subject = yes")
        .await
        .unwrap();
}

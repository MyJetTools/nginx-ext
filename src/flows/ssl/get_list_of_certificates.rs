use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    app::AppContext,
    ssl_certificates::{SslCertificate, SslCertificates},
    storage::nginx::instance::SslCertsPath,
};

pub async fn get_list_of_certificates(app: &AppContext) -> SslCertificates {
    let ssl_cert_path = SslCertsPath::new(&app.settings_reader).await;
    let mut paths = tokio::fs::read_dir(ssl_cert_path.as_str()).await.unwrap(); // Change the path as needed

    let mut result = SslCertificates::new();
    while let Some(path) = paths.next_entry().await.unwrap() {
        let file_name = path.file_name().into_string().unwrap();

        if !file_name.ends_with(".crt") {
            continue;
        }

        let index = file_name.find('.');
        if index.is_none() {
            continue;
        }

        let date_str = &file_name[0..index.unwrap()];

        if date_str.len() != 14 {
            continue;
        }

        let expires_at = DateTimeAsMicroseconds::from_str(date_str);

        if expires_at.is_none() {
            continue;
        }

        let expires_at = expires_at.unwrap();

        let domain = &file_name[index.unwrap() + 1..];

        result.push(SslCertificate {
            domain: domain.replace("%", "*")[0..domain.len() - 4].to_string(),
            expires_at,
            file_name: file_name[0..file_name.len() - 4].to_string(),
        });
    }

    result
}

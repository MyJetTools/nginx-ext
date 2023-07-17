use crate::{ssl_certificates::SslCertificates, storage::model::NginxFileContent};

use super::NginxPath;

pub async fn generate_config_file(
    content: &NginxFileContent,
    ssl_certs: &SslCertificates,
    nginx_path: &NginxPath,
) -> String {
    let file = nginx_path.get_auto_generated_config_file_name();

    let mut result = String::new();

    result.push_str("# Up-streams\n");
    content.generate_nginx_up_streams_configuration(&mut result);

    result.push_str("# Http configurations  \n");
    content.generate_nginx_http_configuration(&mut result, ssl_certs, nginx_path);

    tokio::fs::write(file, result.as_str()).await.unwrap();

    result
}

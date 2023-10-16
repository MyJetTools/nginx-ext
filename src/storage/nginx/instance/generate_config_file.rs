use crate::{ssl_certificates::SslCertificates, storage::model::ConfigFileContent};

use super::NginxPath;

pub async fn generate_config_file(
    content: &ConfigFileContent,
    ssl_certs: &SslCertificates,
    nginx_path: &NginxPath,
) -> String {
    let file = nginx_path.get_auto_generated_config_file_name();

    let mut result = String::new();

    result.push_str("# Up-streams\n");
    content.generate_nginx_up_streams_configuration(&mut result);

    /*
    if content.client_cert_accesses.is_some() {
        result.push_str("# Access lists\n");
        content.generate_access_lists(&mut result);

        result.push_str("\n");
    }
     */

    result.push_str("# Http configurations  \n");
    content.generate_nginx_http_configuration(&mut result, ssl_certs, nginx_path);

    tokio::fs::write(file, result.as_str()).await.unwrap();

    result
}

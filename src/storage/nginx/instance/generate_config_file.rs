use crate::{
    app::AppContext, ssl_certificates::SslCertificates, storage::nginx::models::NginxFileContent,
};

pub async fn generate_config_file(
    app: &AppContext,
    content: &NginxFileContent,
    ssl_certs: &SslCertificates,
) -> String {
    let file = app.settings_reader.get_nginx_config_file_name().await;

    let mut result = String::new();

    result.push_str("# Up-streams\n");
    content.generate_nginx_up_streams_configuration(&mut result);

    result.push_str("# Http configurations  \n");
    content.generate_nginx_http_configuration(&mut result, ssl_certs);

    tokio::fs::write(file, result.as_str()).await.unwrap();

    result
}

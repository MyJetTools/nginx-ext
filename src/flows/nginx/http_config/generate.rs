use crate::app::AppContext;

pub async fn generate(app: &AppContext) -> String {
    let nginx_path = app.settings_reader.get_nginx_path().await;
    let ssl_certs = crate::flows::ssl::get_list_of_certificates(&app).await;
    let mut result = String::new();
    let nginx_content = app.nginx_file_content.read().await;
    nginx_content.generate_nginx_http_configuration(&mut result, &ssl_certs, &nginx_path);

    result
}

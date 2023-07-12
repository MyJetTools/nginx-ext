use crate::app::AppContext;

pub async fn generate_nginx_http_configuration(app: &AppContext) -> String {
    let mut result = String::new();
    let nginx_content = app.nginx_file_content.read().await;
    nginx_content.generate_nginx_http_configuration(&mut result);

    result
}

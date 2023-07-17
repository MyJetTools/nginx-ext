use crate::app::AppContext;

pub async fn generate_nginx_up_streams_configuration(app: &AppContext) -> String {
    let mut result = String::new();
    let nginx_content = app.config_file_content.read().await;
    nginx_content.generate_nginx_up_streams_configuration(&mut result);
    result
}

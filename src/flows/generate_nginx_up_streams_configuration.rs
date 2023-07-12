use crate::app::AppContext;

pub async fn generate_nginx_up_streams_configuration(app: &AppContext) -> String {
    let nginx_content = app.nginx_file_content.read().await;
    nginx_content.generate_nginx_up_streams_configuration()
}

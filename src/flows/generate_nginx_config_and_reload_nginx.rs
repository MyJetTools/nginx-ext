use crate::app::AppContext;

use super::FlowError;

pub async fn generate_nginx_config_and_reload_nginx(app: &AppContext) -> Result<String, FlowError> {
    let ssl_certs = super::ssl::get_list_of_certificates(app).await;

    let nginx_path = app.settings_reader.get_nginx_path().await;

    {
        let nginx_file_content = app.nginx_file_content.read().await;
        crate::storage::nginx::instance::generate_config_file(
            &nginx_file_content,
            &ssl_certs,
            &nginx_path,
        )
        .await;
    }

    match super::reload_nginx(&app.settings_reader).await {
        Ok(result) => Ok(result),
        Err(err) => Err(FlowError::SomethingWentWrong(err)),
    }
}

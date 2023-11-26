use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/nginx/v1/generateConfig",
    summary: "Generate config file",
    description: "Generate config file",
    controller: "Nginx",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct GenerateConfigFileAction {
    app: Arc<AppContext>,
}

impl GenerateConfigFileAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GenerateConfigFileAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let nginx_config = action.app.settings_reader.get_nginx_path().await;
    let ssl_certs = crate::flows::ssl::get_list_of_certificates(&action.app).await;
    let config = {
        let content_file = action.app.config_file_content.read().await;

        crate::storage::nginx::instance::generate_config_file(
            &content_file,
            &ssl_certs,
            &nginx_config,
        )
        .await
    };

    return HttpOutput::as_text(config).into_ok_result(true).into();
}

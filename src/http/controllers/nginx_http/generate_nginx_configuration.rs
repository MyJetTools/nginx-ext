use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/nginx/http/v1/config",
    summary: "Get nginx configuration",
    description: "Get nginx configuration",
    controller: "Nginx Http",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct GetNginxConfigurationAction {
    app: Arc<AppContext>,
}

impl GetNginxConfigurationAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetNginxConfigurationAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::generate_nginx_http_configuration(&action.app).await;
    return HttpOutput::as_text(result).into_ok_result(true).into();
}

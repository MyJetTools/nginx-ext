use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/nginx/v1/reload",
    summary: "Reloads nginx",
    description: "Reloads nginx",
    controller: "Nginx",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct ReloadAction {
    app: Arc<AppContext>,
}

impl ReloadAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ReloadAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    match crate::flows::reload_nginx(&action.app.settings_reader).await {
        Ok(result) => {
            return HttpOutput::as_text(result).into_ok_result(true).into();
        }
        Err(e) => return Err(HttpFailResult::as_fatal_error(e)),
    }
}

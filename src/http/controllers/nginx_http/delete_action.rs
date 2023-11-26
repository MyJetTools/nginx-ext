use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "DELETE",
    route: "/api/nginx/http/v1",
    summary: "Delete Http configuration",
    description: "Delete Http configuration",
    controller: "Nginx Http",
    input_data: "DeleteHttpConfigHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
        {status_code: 403, description: "Configuration not found"}
    ]
)]
pub struct DeleteHttpConfigAction {
    app: Arc<AppContext>,
}

impl DeleteHttpConfigAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DeleteHttpConfigAction,
    input_data: DeleteHttpConfigHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::storage::nginx::http_configs::delete(&action.app, &input_data.domain).await;
    if result {
        let nginx_content = action.app.config_file_content.write().await;
        crate::flows::generate_nginx_config_and_reload_nginx(&action.app, &nginx_content).await?;
        return HttpOutput::Empty.into_ok_result(true).into();
    } else {
        return Err(HttpFailResult::as_forbidden(
            "Upstream not found".to_string().into(),
        ));
    }
}

#[derive(MyHttpInput)]
struct DeleteHttpConfigHttpInputContract {
    #[http_query(description = "Domain name")]
    pub domain: String,
}

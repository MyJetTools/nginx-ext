use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::*;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "DELETE",
    route: "/api/nginx/templates/v1",
    summary: "Delete template",
    description: "Delete template ",
    controller: "Nginx Templates",
    input_data: "DeleteTemplateHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
        {status_code: 403, description: "Template not found"}
    ]
)]
pub struct DeleteTemplateAction {
    app: Arc<AppContext>,
}

impl DeleteTemplateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DeleteTemplateAction,
    input_data: DeleteTemplateHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::storage::nginx::templates::delete(&action.app, &input_data.id).await;
    if result {
        return HttpOutput::Empty.into_ok_result(true).into();
    } else {
        return Err(HttpFailResult::as_forbidden(
            "Upstream not found".to_string().into(),
        ));
    }
}

#[derive(MyHttpInput)]
struct DeleteTemplateHttpInputContract {
    #[http_query(description = "Template name")]
    pub id: String,
}

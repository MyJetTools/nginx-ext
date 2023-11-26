use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/nginx/templates/v1",
    summary: "Insert or replace Http template",
    description: "Insert or replace Http template",
    controller: "Nginx Templates",
    input_data: "InsertOrReplaceTemplateHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct InsertOrReplaceAction {
    app: Arc<AppContext>,
}

impl InsertOrReplaceAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &InsertOrReplaceAction,
    input_data: InsertOrReplaceTemplateHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::storage::nginx::templates::insert_or_replace(
        &action.app,
        input_data.id,
        input_data.lines,
    )
    .await;
    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct InsertOrReplaceTemplateHttpInputContract {
    #[http_body(description = "Id of templates")]
    pub id: String,

    #[http_body(description = "Lines")]
    pub lines: Vec<String>,
}

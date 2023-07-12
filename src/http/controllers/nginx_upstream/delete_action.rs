use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::*;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "DELETE",
    route: "/api/nginx/upstream/v1",
    summary: "Delete upstream configuration",
    description: "Delete upstream configuration",
    controller: "Nginx UpStreams",
    input_data: "DeleteUpstreamHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
        {status_code: 403, description: "Upstream not found"}
    ]
)]
pub struct DeleteUpstreamAction {
    app: Arc<AppContext>,
}

impl DeleteUpstreamAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DeleteUpstreamAction,
    input_data: DeleteUpstreamHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::storage::nginx::up_streams::delete(&action.app, &input_data.name).await;
    if result {
        return HttpOutput::Empty.into_ok_result(true).into();
    } else {
        return Err(HttpFailResult::as_forbidden(
            "Upstream not found".to_string().into(),
        ));
    }
}

#[derive(MyHttpInput)]
struct DeleteUpstreamHttpInputContract {
    #[http_query(description = "Upstream name")]
    pub name: String,
}

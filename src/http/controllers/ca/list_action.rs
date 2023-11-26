use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/ca/v1/list",
    summary: "Get list of CA",
    description: "Get list of CA",
    controller: "Certificate Authority",
    result:[
        {status_code: 200, description: "CA is generated"},
    ]
)]
pub struct GetListOfCaAction {
    app: Arc<AppContext>,
}

impl GetListOfCaAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetListOfCaAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::ca::get_list(&action.app).await;

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

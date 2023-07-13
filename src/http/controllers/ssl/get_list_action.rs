use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use serde::Serialize;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/ssl/v1/list",
    summary: "Get List of SSL certificates",
    description: "Get List of SSL certificates",
    controller: "Ssl",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct GetListOfCertsAction {
    app: Arc<AppContext>,
}

impl GetListOfCertsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetListOfCertsAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let list = crate::flows::ssl::get_list_of_certificates(&action.app).await;

    let result: Vec<_> = list
        .into_inner()
        .into_iter()
        .map(|item| CertificatesHttpModel {
            domain: item.domain,
            expires_at: item.expires_at.to_rfc3339()[..19].to_string(),
        })
        .collect();
    return HttpOutput::as_json(result).into_ok_result(true).into();
}

#[derive(Serialize)]
struct CertificatesHttpModel {
    domain: String,
    expires_at: String,
}

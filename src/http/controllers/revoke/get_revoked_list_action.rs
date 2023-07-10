use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/revoked/v1/list",
    summary: "Get List of revoked certificates",
    description: "Get List of revoked certificates",
    controller: "Revoke",

    result:[
        {status_code: 200, description: "List of revoked certificates", model: "Vec<String>"},

    ]
)]
pub struct GetListOfRevokedCertificatesAction {
    app: Arc<AppContext>,
}

impl GetListOfRevokedCertificatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetListOfRevokedCertificatesAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_list_of_revoked_certificates(&action.app).await;

    let mut http_result = Vec::with_capacity(result.len());

    for item in result {
        let bytes = item.to_be_bytes();

        if item < 256 {
            http_result.push(hex::encode_upper(&bytes[3..4]));
            continue;
        }
        if item < 256 * 256 {
            http_result.push(hex::encode_upper(&bytes[2..4]));
            continue;
        }

        if item < 256 * 256 * 256 {
            http_result.push(hex::encode_upper(&bytes[1..5]));
            continue;
        }

        http_result.push(hex::encode_upper(bytes));
    }
    return HttpOutput::as_json(http_result).into_ok_result(true).into();
}

use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::{app::AppContext, to_hex::ToHex};

#[http_route(
    method: "GET",
    route: "/api/revoked/v1/list",
    summary: "Get List of revoked certificates",
    description: "Get List of revoked certificates",
    controller: "Client Certificates",

    input_data: "GetRevokedCertsInputModel",

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
    input_data: GetRevokedCertsInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result =
        crate::flows::get_list_of_revoked_certificates(&action.app, &input_data.ca_name).await;

    let mut http_result = Vec::with_capacity(result.len());

    for item in result {
        http_result.push(item.to_hex());
    }
    return HttpOutput::as_json(http_result).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct GetRevokedCertsInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}

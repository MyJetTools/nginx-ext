use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/ca/v1/check",
    summary: "Check CA certificate",
    description: "Check CA certificate",
    controller: "Certificate Authority",
    input_data: "CheckCaCertificateInputModel",
    result:[
        {status_code: 200, description: "CA is generated"},
    ]
)]
pub struct CheckCaAction {
    app: Arc<AppContext>,
}

impl CheckCaAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &CheckCaAction,
    input_data: CheckCaCertificateInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca::check_if_we_have_ca_cert(&action.app, &input_data.ca_name).await?;
    return HttpOutput::as_text("Ok".to_string())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
pub struct CheckCaCertificateInputModel {
    #[http_body(name = "caName", description = "Common name")]
    pub ca_name: String,
}

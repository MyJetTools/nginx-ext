use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "DELETE",
    route: "/api/certificates/v1/revoke",
    summary: "Revoke certificate",
    description: "Revoke certificate",
    controller: "Client Certificates",
    input_data: "RevokeClientCertInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct RevokeCertificateAction {
    app: Arc<AppContext>,
}

impl RevokeCertificateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &RevokeCertificateAction,
    input_data: RevokeClientCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::revoke_cert(&action.app, &input_data.ca_name, &input_data.email).await?;

    HttpOutput::Empty.into_ok_result(true)
}

#[derive(MyHttpInput)]
struct RevokeClientCertInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
    #[http_query(name = "email", description = "Email")]
    pub email: String,
}

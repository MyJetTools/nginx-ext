use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/certificates/v1/pem/downloadCert",
    summary: "Download pem certificate",
    description: "Download pem certificate",
    controller: "Certificates",
    input_data: "DownloadPemClientCertInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct DownloadPemCertificateAction {
    app: Arc<AppContext>,
}

impl DownloadPemCertificateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadPemCertificateAction,
    input_data: DownloadPemClientCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result =
        crate::flows::get_pem_cert(&action.app, &input_data.ca_name, &input_data.email).await?;
    return HttpOutput::as_text(String::from_utf8(result).unwrap())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadPemClientCertInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
    #[http_query(name = "email", description = "Email")]
    pub email: String,
}

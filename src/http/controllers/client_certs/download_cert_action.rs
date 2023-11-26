use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/certificates/v1/downloadCert",
    summary: "Download pfx",
    description: "Download pfx",
    controller: "Client Certificates",
    input_data: "DownloadClientCertInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct DownloadCertAction {
    app: Arc<AppContext>,
}

impl DownloadCertAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadCertAction,
    input_data: DownloadClientCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_pfx(
        &action.app,
        &input_data.ca_name,
        &input_data.email,
        &input_data.password,
    )
    .await?;

    return HttpOutput::as_file("cert.pfx".to_string(), result)
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadClientCertInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
    #[http_query(name = "email", description = "Email")]
    pub email: String,
    #[http_query(name = "password", description = "Certificate Password")]
    pub password: String,
}

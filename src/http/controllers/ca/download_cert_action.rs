use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/ca/v1/downloadCert",
    summary: "Download certificate file",
    description: "Download certificate file",
    controller: "Certificate Authority",
    input_data: "DownloadCaCertInputModel",
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
    input_data: DownloadCaCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content =
        crate::storage::ca::load_certificate(&action.app, input_data.ca_name.as_str()).await;

    return HttpOutput::as_text(content.into())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadCaCertInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}

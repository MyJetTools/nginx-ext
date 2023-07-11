use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/ca/v1/downloadRevokedPem",
    summary: "Download revoked pem File",
    description: "Download revoked pem File",
    controller: "Certificate Authority",
    input_data: "DownloadRevokedInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct DownloadRevokedAction {
    app: Arc<AppContext>,
}

impl DownloadRevokedAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadRevokedAction,
    input_data: DownloadRevokedInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let file_name =
        crate::storage::utils::get_crl_pem_file_name(&action.app, &input_data.ca_name).await;

    let content = tokio::fs::read_to_string(file_name).await.unwrap();
    return HttpOutput::as_text(content).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct DownloadRevokedInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}

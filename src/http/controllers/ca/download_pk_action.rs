use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/ca/v1/downloadPk",
    summary: "Get certificate private key pem file",
    description: "Get certificate private key pem file",
    controller: "Certificate Authority",
    input_data: "DownloadCaPrivateKeyInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct DownloadPrivateKeyAction {
    app: Arc<AppContext>,
}

impl DownloadPrivateKeyAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadPrivateKeyAction,
    input_data: DownloadCaPrivateKeyInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content =
        crate::storage::ca::load_private_key(&action.app, input_data.ca_name.as_str()).await;

    return HttpOutput::as_text(content.into())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadCaPrivateKeyInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}

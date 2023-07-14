use std::sync::Arc;

use my_http_server::{types::FileContent, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/ca/v1/import",
    summary: "Import existing CA",
    description: "Import existing CA",
    controller: "Certificate Authority",
    input_data: "ImportCaInputModel",
    result:[
        {status_code: 202, description: "CA is generated"},
    ]
)]
pub struct ImportCaAction {
    app: Arc<AppContext>,
}

impl ImportCaAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &ImportCaAction,
    input_data: ImportCaInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca::import(
        &action.app,
        &input_data.ca_name,
        input_data.private_key.content,
        input_data.cert.content,
    )
    .await;

    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
pub struct ImportCaInputModel {
    #[http_form_data(name = "caName", description = "Common name")]
    pub ca_name: String,

    #[http_form_data(description = "Certificate")]
    pub cert: FileContent,

    #[http_form_data(name="privateKey" description = "Private key")]
    pub private_key: FileContent,
}

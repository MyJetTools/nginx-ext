use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::types::FileContent;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/ssl/v1/upload",
    summary: "Upload Website SSL certificate",
    description: "Upload Website SSL certificate",
    controller: "Ssl",
    input_data: "UploadSslCertificateHttpInput",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct UploadSslCertificateAction {
    app: Arc<AppContext>,
}

impl UploadSslCertificateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &UploadSslCertificateAction,
    input_data: UploadSslCertificateHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ssl::upload_certificate(
        &action.app,
        input_data.certificate.content,
        input_data.private_key.content,
    )
    .await?;
    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct UploadSslCertificateHttpInput {
    #[http_form_data( description: "Pem certificate content")]
    pub certificate: FileContent,

    #[http_form_data(name:"privateKey", description: "Pem certificate content")]
    pub private_key: FileContent,
}

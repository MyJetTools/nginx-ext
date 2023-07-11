use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::{app::AppContext, my_no_sql::ca_entity::ROW_KEY};

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
    let ca = action
        .app
        .ca_my_no_sql_writer
        .get_entity(&input_data.ca_name, ROW_KEY, None)
        .await
        .unwrap();

    if ca.is_none() {
        return Err(HttpFailResult::as_not_found(
            "CA not found".to_string(),
            false,
        ));
    }

    let ca = ca.unwrap();

    return HttpOutput::as_text(String::from_utf8(ca.get_private_key()).unwrap())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadCaPrivateKeyInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}

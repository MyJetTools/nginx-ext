use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/CaAccessList/v1",
    summary: "Insert Client Certificate Access List if not exists",
    description: "Insert Client Certificate Access List if not exists",
    controller: "Client Certificate Accesses",
    input_data: "InsertAccessListIfNotExistsInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct InsertAccessListIfNotExistsAction {
    app: Arc<AppContext>,
}

impl InsertAccessListIfNotExistsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &InsertAccessListIfNotExistsAction,
    input_data: InsertAccessListIfNotExistsInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca_access::list::insert_if_not_exists(
        &action.app,
        &input_data.ca_name,
        &input_data.access_id,
    )
    .await?;

    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct InsertAccessListIfNotExistsInputContract {
    #[http_body(name="caName", description:"Ca name")]
    pub ca_name: String,

    #[http_body(name="accessId", description:"Access Id")]
    pub access_id: String,
}

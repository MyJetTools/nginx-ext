use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/CaUsers/v1/",
    summary: "Add user if not exists",
    description: "Add user if not exists",
    controller: "Client Certificate Users",
    input_data: "AddUserInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct AddUserAction {
    app: Arc<AppContext>,
}

impl AddUserAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &AddUserAction,
    input_data: AddUserInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca_access::users::add_user(
        &action.app,
        &input_data.ca_name,
        &input_data.access_id,
        &input_data.cert_cn,
    )
    .await?;

    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct AddUserInputContract {
    #[http_body(name="caCn", description:"Ca Common name")]
    pub ca_name: String,

    #[http_body(name="accessId", description:"Access Id")]
    pub access_id: String,

    #[http_body(name="certCn", description:"Client certificate Common name")]
    pub cert_cn: String,
}

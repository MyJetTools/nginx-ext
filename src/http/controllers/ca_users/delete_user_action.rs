use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "DELETE",
    route: "/api/CaUsers/v1/",
    summary: "Remove use if exists",
    description: "Remove user if  exists",
    controller: "Client Certificate Users",
    input_data: "DeleteUserInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct DeleteUserAction {
    app: Arc<AppContext>,
}

impl DeleteUserAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DeleteUserAction,
    input_data: DeleteUserInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca_access::users::delete_user(
        &action.app,
        &input_data.ca_name,
        &input_data.access_id,
        &input_data.cert_cn,
    )
    .await?;

    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct DeleteUserInputContract {
    #[http_query(name="caCn", description:"Ca Common name")]
    pub ca_name: String,

    #[http_query(name="accessId", description:"Access Id")]
    pub access_id: String,

    #[http_query(name="certCn", description:"Client certificate Common name")]
    pub cert_cn: String,
}

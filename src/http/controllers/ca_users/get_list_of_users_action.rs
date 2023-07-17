use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/CaUsers/v1/",
    summary: "Get list of users",
    description: "Get list of users",
    controller: "Client Certificate Users",
    input_data: "GetListOfUsersInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct GetListOfUsersAction {
    app: Arc<AppContext>,
}

impl GetListOfUsersAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetListOfUsersAction,
    input_data: GetListOfUsersInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::ca_access::users::get_list(
        &action.app,
        &input_data.ca_name,
        &input_data.access_id,
    )
    .await?;

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct GetListOfUsersInputContract {
    #[http_query(name="caCn", description:"Ca Common name")]
    pub ca_name: String,

    #[http_query(name="accessId", description:"Access Id")]
    pub access_id: String,
}

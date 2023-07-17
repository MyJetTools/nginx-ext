use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;

use crate::app::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/CaUsersList/v1/",
    summary: "Get Client Certificate Access List",
    description: "Get Client Certificate Access List",
    controller: "Client Certificate Accesses",
    input_data: "GetAccessListHttpContract",
    result:[
        {status_code: 200, description: "Ok result"},
    ]
)]
pub struct GetAccessListAction {
    app: Arc<AppContext>,
}

impl GetAccessListAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetAccessListAction,
    input_data: GetAccessListHttpContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::ca_access::list::get_list(&action.app, &input_data.ca_name).await?;

    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct GetAccessListHttpContract {
    #[http_query(name="caName", description:"Ca name")]
    pub ca_name: String,
}

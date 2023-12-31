use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use super::models::*;
use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/nginx/http/v1",
    summary: "Insert or replace Http configuration",
    description: "Insert or replace Http configuration",
    controller: "Nginx Http",
    input_data: "HttpConfigurationHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct InsertOrReplaceAction {
    app: Arc<AppContext>,
}

impl InsertOrReplaceAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &InsertOrReplaceAction,
    input_data: HttpConfigurationHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let (name, config) = input_data.into_storage_model()?;

    crate::flows::nginx::http_config::insert_or_replace(&action.app, name, config).await?;

    return HttpOutput::Empty.into_ok_result(true).into();
}

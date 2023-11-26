use std::{collections::BTreeMap, sync::Arc};

use my_http_server::macros::*;
use my_http_server::*;

use serde::Serialize;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/nginx/templates/v1",
    summary: "Get templates",
    description: "Get templates we have",
    controller: "Nginx Templates",
    result:[
        {status_code: 200, description: "Ok result", model: "GetTemplatesResponse"},
    ]
)]
pub struct GetTemplatesAction {
    app: Arc<AppContext>,
}

impl GetTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetTemplatesAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let templates = crate::storage::nginx::templates::get_templates(&action.app).await;

    let templates: BTreeMap<String, Vec<String>> = templates.unwrap_or_default();

    return HttpOutput::as_json(templates).into_ok_result(true).into();
}

#[derive(MyHttpObjectStructure, Serialize)]
struct GetTemplatesResponse {
    pub templates: BTreeMap<String, Vec<String>>,
}

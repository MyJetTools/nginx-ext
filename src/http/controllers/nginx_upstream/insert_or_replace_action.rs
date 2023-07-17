use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::*;
use serde::Deserialize;

use crate::{app::AppContext, storage::model::UpStreamRouteStorageModel};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/nginx/upstream/v1",
    summary: "Insert or replace upstream configuration",
    description: "Insert or replace upstream configuration",
    controller: "Nginx UpStreams",
    input_data: "InsertOrReplaceUpstreamHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct InsertOrReplaceUpstreamAction {
    app: Arc<AppContext>,
}

impl InsertOrReplaceUpstreamAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &InsertOrReplaceUpstreamAction,
    input_data: InsertOrReplaceUpstreamHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let (name, routes) = input_data.into_storage_model();
    crate::storage::nginx::up_streams::insert_or_replace(&action.app, name, routes).await;
    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct InsertOrReplaceUpstreamHttpInputContract {
    #[http_body(description = "Upstream name")]
    pub name: String,
    #[http_body(description = "Upstream routes")]
    pub routes: Vec<UpstreamHttpModel>,
}

impl InsertOrReplaceUpstreamHttpInputContract {
    pub fn into_storage_model(self) -> (String, Vec<UpStreamRouteStorageModel>) {
        let mut result = Vec::with_capacity(self.routes.len());

        for r in self.routes {
            result.push(r.into());
        }

        (self.name, result)
    }
}

#[derive(MyHttpInputObjectStructure, Deserialize)]
struct UpstreamHttpModel {
    #[serde(rename = "remoteAddr")]
    pub remote_addr: String,
    pub weight: Option<u32>,
    #[serde(rename = "isBackup")]
    pub is_backup: bool,
}

impl Into<UpStreamRouteStorageModel> for UpstreamHttpModel {
    fn into(self) -> UpStreamRouteStorageModel {
        UpStreamRouteStorageModel {
            remote_addr: self.remote_addr,
            weight: self.weight,
            is_backup: self.is_backup,
        }
    }
}

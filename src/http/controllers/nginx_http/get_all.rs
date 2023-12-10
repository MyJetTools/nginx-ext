use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

use super::models::*;

#[http_route(
    method: "GET",
    route: "/api/nginx/http/v1/all",
    summary: "Get all Http configurations",
    description: "Get all Http configurations",
    controller: "Nginx Http",
    input_data: "GetHttpConfigurationHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result", model:"Vec<HttpConfigurationHttpInputContract>"},
    ]
)]
pub struct GetAllAction {
    app: Arc<AppContext>,
}

impl GetAllAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetAllAction,
    input_data: GetHttpConfigurationHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = {
        let mut result = Vec::new();
        let read_access = action.app.config_file_content.read().await;

        match &read_access.http_configs {
            Some(configs) => {
                for config in configs.values() {
                    let item = HttpConfigurationHttpInputContract {
                        domain: input_data.domain.clone(),
                        port: config.port,
                        protocol: config.protocol.to_string().to_string(),
                        ssl_certificate: config.ssl_cert.clone(),
                        client_cert_ca_cn: config.ca_cn.clone(),

                        templates: config.templates.clone(),
                        locations: config
                            .locations
                            .iter()
                            .map(|l| HttpLocationHttpModel {
                                location: l.location.clone(),
                                proxy_pass: l.proxy_pass.clone(),
                                templates: l.templates.clone(),
                            })
                            .collect(),
                    };
                    result.push(item);
                }

                result
            }
            None => result,
        }
    };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
pub struct GetHttpConfigurationHttpInputContract {
    #[http_query(description = "Domain name")]
    pub domain: String,
}
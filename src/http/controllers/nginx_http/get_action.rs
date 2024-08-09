use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

use super::models::*;

#[http_route(
    method: "GET",
    route: "/api/nginx/http/v1",
    summary: "Get Http configuration",
    description: "Get Http configuration",
    controller: "Nginx Http",
    input_data: "GetHttpConfigurationHttpInputContract",
    result:[
        {status_code: 202, description: "Ok result", model:"HttpConfigurationHttpInputContract"},
    ]
)]
pub struct GetAction {
    app: Arc<AppContext>,
}

impl GetAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetAction,
    input_data: GetHttpConfigurationHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let config = {
        let read_access = action.app.config_file_content.read().await;

        match &read_access.http_configs {
            Some(configs) => {
                if let Some(config) = configs.get(&input_data.domain) {
                    Some(HttpConfigurationHttpInputContract {
                        domain: input_data.domain,
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
                                raw_lines: l.raw_lines.clone(),
                                http2: l.http2,
                            })
                            .collect(),
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    };

    if let Some(config) = config {
        return HttpOutput::as_json(config).into_ok_result(true).into();
    } else {
        return Err(HttpFailResult::as_forbidden("Not found".to_string().into()));
    }
}

#[derive(MyHttpInput)]
pub struct GetHttpConfigurationHttpInputContract {
    #[http_query(description = "Domain name")]
    pub domain: String,
}

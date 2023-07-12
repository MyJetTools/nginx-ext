use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::*;
use serde::Deserialize;

use crate::{app::AppContext, storage::nginx::models::*};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/nginx/http/v1",
    summary: "Insert or replace Http configuration",
    description: "Insert or replace Http configuration",
    controller: "Nginx Http",
    input_data: "InsertOrReplaceHttpConfigurationHttpInputContract",
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
    input_data: InsertOrReplaceHttpConfigurationHttpInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let (name, config) = input_data.into_storage_model()?;
    crate::storage::nginx::http_configs::insert_or_replace(&action.app, name, config).await;
    return HttpOutput::Empty.into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct InsertOrReplaceHttpConfigurationHttpInputContract {
    #[http_body(description = "Domain name")]
    pub domain: String,

    #[http_body(description = "Listen port")]
    pub port: u16,

    #[http_body(description = "Protocol: Http, Https, Https2")]
    pub protocol: String,

    #[http_body(name="sslCertificate" description = "Ssl certificate name, or Null/Empty if not used")]
    pub ssl_certificate: Option<String>,

    #[http_body(description = "Upstream routes")]
    pub locations: Vec<HttpLocationHttpModel>,
}

impl InsertOrReplaceHttpConfigurationHttpInputContract {
    pub fn into_storage_model(self) -> Result<(String, HttpConfig), HttpFailResult> {
        let protocol = match self.protocol.as_str() {
            "Http" => HttpProtocol::Http,
            "Https" => HttpProtocol::Https,
            "Https2" => HttpProtocol::Https2,
            _ => {
                return Err(HttpFailResult::as_validation_error(
                    "Protocol field must be Http, Https, Https2".to_string(),
                ))
            }
        };

        let result = HttpConfig {
            protocol: protocol,
            port: self.port,
            ssl_cert: if let Some(ssl_cert) = self.ssl_certificate {
                if ssl_cert == "" {
                    None
                } else {
                    Some(ssl_cert)
                }
            } else {
                None
            },
            locations: self.locations.into_iter().map(|x| x.into()).collect(),
        };

        Ok((self.domain, result))
    }
}

#[derive(MyHttpInputObjectStructure, Deserialize)]
struct HttpLocationHttpModel {
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "proxyPass")]
    pub proxy_pass: String,
}

impl Into<HttpConfigLocation> for HttpLocationHttpModel {
    fn into(self) -> HttpConfigLocation {
        HttpConfigLocation {
            location: self.location,
            proxy_pass: self.proxy_pass,
        }
    }
}

use my_http_server::macros::*;
use my_http_server::*;

use serde::{Deserialize, Serialize};

use crate::storage::model::*;

#[derive(MyHttpInput, Serialize, MyHttpObjectStructure)]
pub struct HttpConfigurationHttpInputContract {
    #[http_body(description = "Domain name")]
    pub domain: String,

    #[http_body(description = "Listen port")]
    pub port: u16,

    #[http_body(description = "Protocol: Http, Https, Https2")]
    pub protocol: String,

    #[http_body(name="sslCertificate" description = "Ssl certificate name, or Null/Empty if not used")]
    #[serde(rename = "sslCertificate")]
    pub ssl_certificate: Option<String>,

    #[http_body(name="clientCertCaCn" description = "Client Certificate CA Common Name")]
    #[serde(rename = "clientCertCaCn")]
    pub client_cert_ca_cn: Option<String>,

    #[http_body(description = "List of templates")]
    pub templates: Option<Vec<String>>,

    #[http_body(description = "Upstream routes")]
    pub locations: Vec<HttpLocationHttpModel>,
}

impl HttpConfigurationHttpInputContract {
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
            templates: self.templates,
            ca_cn: self.client_cert_ca_cn,
            locations: self.locations.into_iter().map(|x| x.into()).collect(),
        };

        Ok((self.domain, result))
    }
}

#[derive(MyHttpInputObjectStructure, Deserialize, Serialize)]
pub struct HttpLocationHttpModel {
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "proxyPass")]
    pub proxy_pass: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<bool>,

    #[serde(rename = "rawLines")]
    pub raw_lines: Option<Vec<String>>,

    pub templates: Option<Vec<String>>,
}

impl Into<HttpConfigLocation> for HttpLocationHttpModel {
    fn into(self) -> HttpConfigLocation {
        HttpConfigLocation {
            location: self.location,
            proxy_pass: self.proxy_pass,
            templates: self.templates,
            raw_lines: self.raw_lines,
            http2: self.http2,
        }
    }
}

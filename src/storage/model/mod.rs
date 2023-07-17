mod http_configs;
mod upstreams;
pub use http_configs::*;
use serde::*;
use std::collections::HashMap;
pub use upstreams::*;

use crate::ssl_certificates::SslCertificates;

use super::nginx::instance::NginxPath;

#[derive(Default, Deserialize, Serialize)]
pub struct NginxFileContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<HashMap<String, Vec<UpStreamRouteStorageModel>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_configs: Option<HashMap<String, HttpConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<HashMap<String, Vec<String>>>,
}

impl NginxFileContent {
    pub fn generate_nginx_up_streams_configuration(&self, dest: &mut String) {
        if let Some(upstrams) = self.upstreams.as_ref() {
            for (name, routes) in upstrams {
                dest.push_str("upstream ");
                dest.push_str(name);
                dest.push_str(" {\n");

                for rt in routes {
                    dest.push_str("  server ");
                    dest.push_str(rt.remote_addr.as_str());

                    if let Some(w) = rt.weight {
                        dest.push_str(" weight=");
                        dest.push_str(w.to_string().as_str());
                    }

                    if rt.is_backup {
                        dest.push_str(" backup");
                    }

                    dest.push_str(";\n")
                }

                dest.push_str("}\n\n");
            }
        }
    }

    pub fn generate_nginx_http_configuration(
        &self,
        dest: &mut String,
        ssl_certs: &SslCertificates,
        nginx_path: &NginxPath,
    ) {
        if let Some(http_configs) = self.http_configs.as_ref() {
            for (domain, http_config) in http_configs {
                http_config.generate_nginx_configuration(
                    domain,
                    dest,
                    &self.templates,
                    ssl_certs,
                    nginx_path,
                );
            }
        }
    }
}

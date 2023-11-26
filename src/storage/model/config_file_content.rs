use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{ssl_certificates::SslCertificates, storage::nginx::instance::NginxPath};

use super::{HttpConfig, UpStreamRouteStorageModel};

#[derive(Default, Deserialize, Serialize)]
pub struct ConfigFileContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<HashMap<String, Vec<UpStreamRouteStorageModel>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_configs: Option<HashMap<String, HttpConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_accesses: Option<HashMap<String, HashMap<String, Vec<String>>>>,
}

impl ConfigFileContent {
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

    pub fn generate_access_lists(&self, dest: &mut String) {
        if self.client_cert_accesses.is_none() {
            return;
        }

        let client_cert_accesses = self.client_cert_accesses.as_ref().unwrap();

        for (ca_name, access_list) in client_cert_accesses {
            for (access_list_name, users) in access_list {
                dest.push_str("map $ssl_client_s_dn_cn $ca_");
                dest.push_str(ca_name);
                dest.push_str("_");
                dest.push_str(access_list_name);
                dest.push_str("{\n default 1;\n");

                for user in users {
                    dest.push('"');
                    dest.push_str(user);
                    dest.push('"');
                    dest.push_str(" 0;\n");
                }

                dest.push_str("}\n");
            }
        }
    }

    pub fn has_config_file_content(&self, my_ca_name: &str, my_access_list_name: &str) -> bool {
        if self.client_cert_accesses.is_none() {
            return false;
        }

        let client_cert_accesses = self.client_cert_accesses.as_ref().unwrap();

        if let Some(access_list) = client_cert_accesses.get(my_ca_name) {
            for access_is in access_list.keys() {
                if access_is == my_access_list_name {
                    return true;
                }
            }
        }

        false
    }

    pub fn insert_access_list(&mut self, my_ca_name: &str, my_access_list_name: &str) {
        if self.client_cert_accesses.is_none() {
            self.client_cert_accesses = Some(HashMap::new());
        }

        let client_cert_accesses = self.client_cert_accesses.as_mut().unwrap();

        client_cert_accesses
            .entry(my_ca_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(my_access_list_name.to_string(), Vec::new());
    }
}

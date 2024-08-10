use std::collections::BTreeMap;

use rust_extensions::{date_time::DateTimeAsMicroseconds, StrOrString};
use serde::{Deserialize, Serialize};

use crate::{ssl_certificates::SslCertificates, storage::nginx::instance::NginxPath};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpProtocol {
    Http,
    Https,
    Https2,
}

impl HttpProtocol {
    pub fn generate_nginx_configuration(&self, dest: &mut String) {
        match self {
            HttpProtocol::Http => {}
            HttpProtocol::Https => dest.push_str(" ssl "),
            HttpProtocol::Https2 => dest.push_str(" ssl http2"),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            HttpProtocol::Http => "Http",
            HttpProtocol::Https => "Https",
            HttpProtocol::Https2 => "Https2",
        }
    }

    pub fn is_https(&self) -> bool {
        match self {
            HttpProtocol::Http => false,
            HttpProtocol::Https => true,
            HttpProtocol::Https2 => true,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub protocol: HttpProtocol,
    pub port: u16,
    pub locations: Vec<HttpConfigLocation>,
    pub ssl_cert: Option<String>,
    pub ca_cn: Option<String>,
    pub templates: Option<Vec<String>>,
}

impl HttpConfig {
    pub fn generate_nginx_configuration(
        &self,
        domain: &str,
        dest: &mut String,
        templates_repo: &Option<BTreeMap<String, Vec<String>>>,
        ssl_certs: &SslCertificates,
        nginx_path: &NginxPath,
    ) {
        let now = DateTimeAsMicroseconds::now();
        dest.push_str("server {\n");

        dest.push_str(" listen ");
        dest.push_str(self.port.to_string().as_str());
        self.protocol.generate_nginx_configuration(dest);
        dest.push_str(";\n");

        dest.push_str(" server_name ");
        dest.push_str(domain);
        dest.push_str(";\n");

        let ssl_cert = if let Some(ssl_cert) = &self.ssl_cert {
            Some(StrOrString::create_as_str(ssl_cert))
        } else {
            if self.protocol.is_https() {
                let cert_by_domain: Option<&crate::ssl_certificates::SslCertificate> =
                    ssl_certs.get_by_domain(domain, now);

                if let Some(cert) = cert_by_domain {
                    Some(StrOrString::create_as_str(cert.file_name.as_str()))
                } else {
                    Some(StrOrString::create_as_str(
                        crate::storage::nginx::instance::SELF_SIGNED_CERT_NAME,
                    ))
                }
            } else {
                None
            }
        };

        let certs_path = nginx_path.get_certs_path();

        if let Some(ssl_cert) = ssl_cert {
            dest.push_str("\n ssl_certificate   ");
            dest.push_str(certs_path.as_str());
            dest.push_str(ssl_cert.as_str());
            dest.push_str(".crt;\n");

            dest.push_str(" ssl_certificate_key ");
            dest.push_str(certs_path.as_str());
            dest.push_str(ssl_cert.as_str());
            dest.push_str(".key;\n");
        }

        if let Some(ca_cn) = &self.ca_cn {
            dest.push_str(" ssl_client_certificate ");
            dest.push_str(certs_path.as_str());
            dest.push_str("ca_");
            dest.push_str(ca_cn);
            dest.push_str(".crt;\n");

            dest.push_str(" ssl_verify_client on;\n");
        }

        dest.push_str("\n access_log off;\n");
        dest.push_str(" error_log off;\n");

        render_templates(dest, &self.templates, templates_repo, 1);

        for location in &self.locations {
            location.generate_nginx_configuration(dest, templates_repo);
        }

        dest.push_str("\n}\n");
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfigLocation {
    pub location: String,
    pub proxy_pass: Option<String>,
    pub raw_lines: Option<Vec<String>>,
    pub templates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stub_status: Option<bool>,
}

impl HttpConfigLocation {
    pub fn generate_nginx_configuration(
        &self,
        dest: &mut String,
        templates_repo: &Option<BTreeMap<String, Vec<String>>>,
    ) {
        dest.push_str("\n location ");
        dest.push_str(self.location.as_str());
        dest.push_str("  {\n");

        if let Some(proxy_pass) = &self.proxy_pass {
            dest.push_str("  proxy_pass ");
            dest.push_str(proxy_pass);
            dest.push_str(";\n");
        }

        if let Some(http2) = self.http2 {
            if http2 {
                dest.push_str("  proxy_http_version 2;\n");
            }
        }

        if let Some(stub_status) = self.stub_status {
            if stub_status {
                dest.push_str("  stub_status;\n");
            }
        }

        if let Some(raw_lines) = &self.raw_lines {
            for raw_line in raw_lines {
                dest.push_str(raw_line);
                dest.push_str("\n");
            }
        }

        render_templates(dest, &self.templates, templates_repo, 2);

        dest.push_str(" }\n");
    }
}

fn render_templates(
    dest: &mut String,
    templates: &Option<Vec<String>>,
    templates_repo: &Option<BTreeMap<String, Vec<String>>>,
    offset: usize,
) {
    if templates.is_none() {
        return;
    }

    let templates = templates.as_ref().unwrap();

    if templates.len() == 0 {
        return;
    }

    if let Some(templates_repo) = templates_repo {
        for template_id in templates {
            if let Some(template) = templates_repo.get(template_id) {
                dest.push_str("\n");
                for _ in 0..offset {
                    dest.push(' ');
                }
                dest.push_str(format!("# Template {}\n", template_id).as_str());
                for line in template {
                    for _ in 0..offset {
                        dest.push(' ');
                    }
                    dest.push_str(line.as_str());
                    dest.push_str("\n");
                }

                dest.push_str("\n");
            } else {
                dest.push_str(format!("# Template {}  is not found\n", template_id).as_str());
            }
        }
    } else {
        dest.push_str(format!("# Templates {:?} are not found\n", templates).as_str());
    }
}

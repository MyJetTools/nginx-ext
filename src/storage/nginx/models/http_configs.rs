use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub protocol: HttpProtocol,
    pub port: u16,
    pub locations: Vec<HttpConfigLocation>,
    pub ssl_cert: Option<String>,

    pub templates: Option<Vec<String>>,
}

impl HttpConfig {
    pub fn generate_nginx_configuration(
        &self,
        domain: &str,
        dest: &mut String,
        templates_repo: &Option<HashMap<String, Vec<String>>>,
    ) {
        dest.push_str("server {\n");

        dest.push_str(" listen ");
        dest.push_str(self.port.to_string().as_str());
        self.protocol.generate_nginx_configuration(dest);
        dest.push_str(";\n");

        dest.push_str(" server_name ");
        dest.push_str(domain);
        dest.push_str(";\n");

        if let Some(ssl_cert) = &self.ssl_cert {
            dest.push_str("\n ssl_certificate   /etc/nginx/certs/");
            dest.push_str(ssl_cert.as_str());
            dest.push_str(".crt;\n");

            dest.push_str(" ssl_certificate_key /etc/nginx/certs/");
            dest.push_str(ssl_cert.as_str());
            dest.push_str(".key;\n");
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
    pub proxy_pass: String,
    pub templates: Option<Vec<String>>,
}

impl HttpConfigLocation {
    pub fn generate_nginx_configuration(
        &self,
        dest: &mut String,
        templates_repo: &Option<HashMap<String, Vec<String>>>,
    ) {
        dest.push_str("\n location ");
        dest.push_str(self.location.as_str());
        dest.push_str("  {\n");

        dest.push_str("  proxy_pass ");
        dest.push_str(self.proxy_pass.as_str());
        dest.push_str(";\n");

        render_templates(dest, &self.templates, templates_repo, 2);

        dest.push_str(" }\n");
    }
}

fn render_templates(
    dest: &mut String,
    templates: &Option<Vec<String>>,
    templates_repo: &Option<HashMap<String, Vec<String>>>,
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

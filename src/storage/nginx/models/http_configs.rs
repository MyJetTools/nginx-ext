use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpConfig {
    pub protocol: HttpProtocol,
    pub port: u16,
    pub locations: Vec<HttpConfigLocation>,
    pub ssl_cert: Option<String>,
}

impl HttpConfig {
    pub fn generate_nginx_configuration(&self, domain: &str, dest: &mut String) {
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

        for location in &self.locations {
            location.generate_nginx_configuration(dest);
        }

        dest.push_str("\n}\n");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpConfigLocation {
    pub location: String,
    pub proxy_pass: String,
}

impl HttpConfigLocation {
    pub fn generate_nginx_configuration(&self, dest: &mut String) {
        dest.push_str("\n location ");
        dest.push_str(self.location.as_str());
        dest.push_str("  {\n");

        dest.push_str("  proxy_pass ");
        dest.push_str(self.proxy_pass.as_str());
        dest.push_str(";\n");

        dest.push_str(" }\n");
    }
}

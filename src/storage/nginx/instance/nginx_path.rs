//const NGINX_PATH: &str = "/etc/nginx/";
pub struct NginxPath {
    path: String,
}

impl NginxPath {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn get_certs_path(&self) -> String {
        let mut result = self.path.clone();
        result.push_str("certs/");
        result
    }

    pub fn get_self_signed_private_key_file_name(&self) -> String {
        let mut result = self.get_certs_path();
        result.push_str("self.key");
        result
    }

    pub fn get_self_signed_cert_file_name(&self) -> String {
        let mut result = self.get_certs_path();
        result.push_str("self.crt");
        result
    }

    pub fn get_ca_cert_file(&self, ca_cn: &str) -> String {
        let mut result = self.get_certs_path();
        result.push_str("ca_");
        result.push_str(ca_cn);
        result.push_str(".crt");
        result
    }

    pub fn get_ssl_cert_file_path(&self, file_name: &str) -> String {
        let mut result = self.get_certs_path();
        result.push_str(file_name);
        result.push_str(".crt");
        result
    }

    pub fn get_ssl_private_key_file_path(&self, file_name: &str) -> String {
        let mut result = self.get_certs_path();
        result.push_str(file_name);
        result.push_str(".key");
        result
    }

    pub fn get_config_file(&self) -> String {
        let mut result = self.path.clone();
        result.push_str("nginx.conf");
        result
    }

    // Generated Http Content
    pub fn get_path_to_generate_http_content(&self) -> String {
        let mut result = self.path.clone();
        result.push_str("sites-enabled/");
        result
    }

    pub fn get_default_http_configuration_file(&self) -> String {
        let mut result = self.get_path_to_generate_http_content();
        result.push_str("default-site.conf");
        result
    }

    pub fn get_auto_generated_config_file_name(&self) -> String {
        let mut result = self.get_path_to_generate_http_content();
        result.push_str("auto-generated.conf");
        result
    }

    // Generated Tcp Content
    pub fn get_path_to_generate_tcp_content(&self) -> String {
        let mut result = self.path.clone();
        result.push_str("streams/");
        result
    }
}

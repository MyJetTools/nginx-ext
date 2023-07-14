//const NGINX_PATH: &str = "/etc/nginx/";
pub struct NginxPath {
    path: String,
}

impl NginxPath {
    pub fn new(path: String) -> Self {
        Self { path }
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

    pub fn get_default_configuration_file(&self) -> String {
        let mut result = self.path.clone();
        result.push_str("sites-enabled/default-site.conf");
        result
    }

    pub fn get_auto_generated_config_file_name(&self) -> String {
        let mut result: String = self.path.clone();
        result.push_str("sites-enabled/auth-generated.conf");
        result
    }
}

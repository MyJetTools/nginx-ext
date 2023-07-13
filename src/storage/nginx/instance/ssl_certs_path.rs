use crate::settings::SettingsReader;

pub struct SslCertsPath {
    path: String,
}

impl SslCertsPath {
    pub async fn new(settings: &SettingsReader) -> Self {
        let path = settings.get_nginx_certs_path().await;
        Self { path }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn generate_ca_cert_file(&self, ca_cn: &str) -> String {
        let mut result = self.path.clone();
        result.push_str("ca_");
        result.push_str(ca_cn);
        result.push_str(".crt");
        result
    }

    pub fn generate_certificate_file(&self, name: &str) -> String {
        let mut result = self.path.clone();
        result.push_str(name);
        result.push_str(".crt");
        result
    }

    pub fn generate_private_key_file(&self, name: &str) -> String {
        let mut result = self.path.clone();
        result.push_str(name);
        result.push_str(".key");
        result
    }
}

use crate::{app::AppContext, storage::ca::CaPath};
pub const CERT_FILE_NAME: &str = "cert.pem";
pub const PUBLIC_KEY_FILE_NAME: &str = "public_key.pem";
pub const PRIVATE_KEY_FILE_NAME: &str = "private_key.pem";

#[derive(Clone)]
pub struct CertPath {
    path: String,
}

impl CertPath {
    pub async fn new(app: &AppContext, cn_name: &str, email: &str) -> Self {
        let path = CaPath::new(app, cn_name).await;
        Self::from_ca_path(path, email)
    }

    pub fn from_ca_path(path: CaPath, email: &str) -> Self {
        let mut path: String = path.into();
        let sub_path = email.replace("@", "_");

        path.push_str("/certs/");
        path.push_str(sub_path.as_str());

        Self { path }
    }

    fn into_file_name(self, file_name: &str) -> String {
        let mut result = self.path;

        if !result.ends_with('/') {
            result.push('/');
        }

        result.push_str(file_name);

        result
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn into_cert_file_name(self) -> String {
        self.into_file_name(CERT_FILE_NAME)
    }

    pub fn into_private_key_file_name(self) -> String {
        self.into_file_name(PRIVATE_KEY_FILE_NAME)
    }

    /*
    pub fn into_public_key_file_name(self) -> String {
        self.into_file_name(PUBLIC_KEY_FILE_NAME)
    }
     */

    pub fn to_cert_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(CERT_FILE_NAME)
    }

    pub fn to_private_key_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(PRIVATE_KEY_FILE_NAME)
    }

    pub fn to_public_key_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(PUBLIC_KEY_FILE_NAME)
    }
}

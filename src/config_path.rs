use crate::storage::ca::CaDataPath;

pub struct ConfigPath {
    path: String,
}

impl ConfigPath {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn into_ca_path(self) -> String {
        let mut path = self.path;
        path.push_str("Ca/");
        path
    }

    /*
       pub fn to_ca_path(&self) -> String {
           let mut path = self.path.clone();
           path.push_str("Ca/");
           path
       }
    */
    pub fn into_ca_data_path(self, ca_cn: &str) -> CaDataPath {
        CaDataPath::new(self.into_ca_path(), ca_cn)
    }

    /*
       pub fn get_nginx_yaml_config_file_name(&self) -> String {
           let mut path = self.path.clone();
           path.push_str("nginx.yaml");
           path
       }
    */
    pub fn into_nginx_yaml_config_file_name(self) -> String {
        let mut path = self.path;
        path.push_str("nginx.yaml");
        path
    }
}

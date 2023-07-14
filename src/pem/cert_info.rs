#[derive(Debug)]
pub struct PemCertInfo {
    pub ca_cn: String,
    pub organization: String,
    pub country_code: String,
    pub city: String,
}

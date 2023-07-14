use std::sync::Arc;

use openssl::x509::X509Crl;

use crate::app::AppContext;

pub async fn get_list_of_revoked_certificates(app: &Arc<AppContext>, ca_cn: &str) -> Vec<u32> {
    let ca_path = app
        .settings_reader
        .get_config_path()
        .await
        .into_ca_data_path(ca_cn);

    let revoked_file_name = ca_path.into_crl_file_name();
    let result = tokio::fs::read_to_string(revoked_file_name).await.unwrap();

    let crl = X509Crl::from_pem(result.as_bytes()).unwrap();

    let mut result = Vec::new();

    for revoked in crl.get_revoked().unwrap() {
        let number = revoked.serial_number();

        let value = format!("{}", number.to_bn().unwrap());
        result.push(value.parse().unwrap());
    }

    result
}

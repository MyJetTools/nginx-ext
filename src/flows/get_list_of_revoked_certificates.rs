use std::sync::Arc;

use openssl::x509::X509Crl;

use crate::app::AppContext;

pub async fn get_list_of_revoked_certificates(app: &Arc<AppContext>) -> Vec<u32> {
    let revoke_file_path = app.settings_reader.get_revoked_crl_pem_file().await;

    let result = tokio::fs::read_to_string(revoke_file_path).await.unwrap();

    let crl = X509Crl::from_pem(result.as_bytes()).unwrap();

    let mut result = Vec::new();

    for revoked in crl.get_revoked().unwrap() {
        let number = revoked.serial_number();

        let value = format!("{}", number.to_bn().unwrap());
        result.push(value.parse().unwrap());
    }

    result
}

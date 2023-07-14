use tokio::process::Command;

use crate::{app::AppContext, storage::cert::CertPath};

use super::FlowError;

pub async fn revoke_cert(app: &AppContext, ca_cn: &str, email: &str) -> Result<(), FlowError> {
    let ca_path = app.settings_reader.get_ca_data_path(ca_cn.into()).await;

    let ca_private_key_file_name = ca_path.to_private_key_file_name();

    let ca_cert_file_name = ca_path.to_cert_file_name();
    let config_file_name = ca_path.to_config_file_name();

    let crl_pem_file = ca_path.to_crl_file_name();

    let cert_path = CertPath::from_ca_path(ca_path, email);

    //openssl ca -config ./openssl.cnf -keyfile ca_private.key -cert ca_cert.pem -revoke cert_to_revoke.pem

    let output = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg(config_file_name.as_str())
        .arg("-keyfile")
        .arg(ca_private_key_file_name.as_str())
        .arg("-cert")
        .arg(ca_cert_file_name.as_str())
        .arg("-revoke")
        .arg(cert_path.into_cert_file_name())
        .output()
        .await
        .unwrap();

    if !output.status.success() {
        return Err(FlowError::SomethingWentWrong(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    //openssl ca -config ./openssl.cnf -keyfile ca_private.key -cert ca_cert.pem -revoke cert_to_revoke.pem

    let output = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg(config_file_name.as_str())
        .arg("-keyfile")
        .arg(ca_private_key_file_name)
        .arg("-cert")
        .arg(ca_cert_file_name)
        .arg("-gencrl")
        .arg("-out")
        .arg(crl_pem_file)
        .output()
        .await
        .unwrap();

    if !output.status.success() {
        return Err(FlowError::SomethingWentWrong(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

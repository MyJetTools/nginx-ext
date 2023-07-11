use tokio::process::Command;

use crate::{app::AppContext, storage::utils::CERT_FILE_NAME};

use super::FlowError;

pub async fn revoke_cert(app: &AppContext, ca_cn: &str, email: &str) -> Result<(), FlowError> {
    let cert_path = crate::storage::cert::compile_cert_path(app, ca_cn, email).await;

    let cert_file_name = format!("{}/{}", cert_path, CERT_FILE_NAME);

    println!("Revoking certificate: {}", cert_path);

    let ca_key_path = crate::storage::utils::get_ca_private_key_file_name(app, ca_cn).await;

    let ca_cert_path = crate::storage::utils::get_ca_cert_file_name(app, ca_cn).await;
    let config_file_name = crate::storage::utils::get_config_name(app, ca_cn).await;

    //openssl ca -config ./openssl.cnf -keyfile ca_private.key -cert ca_cert.pem -revoke cert_to_revoke.pem

    let output = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg(config_file_name.as_str())
        .arg("-keyfile")
        .arg(ca_key_path.as_str())
        .arg("-cert")
        .arg(ca_cert_path.as_str())
        .arg("-revoke")
        .arg(cert_file_name.as_str())
        .output()
        .await
        .unwrap();

    if !output.status.success() {
        return Err(FlowError::SomethingWentWrong(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    //openssl ca -config ./openssl.cnf -keyfile ca_private.key -cert ca_cert.pem -revoke cert_to_revoke.pem

    let ca_private_key_file = crate::storage::utils::get_ca_private_key_file_name(app, ca_cn).await;
    let ca_cert_file = crate::storage::utils::get_ca_cert_file_name(app, ca_cn).await;

    let crl_pem = crate::storage::utils::get_crl_pem_file_name(app, ca_cn).await;

    let output = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg(config_file_name.as_str())
        .arg("-keyfile")
        .arg(ca_private_key_file)
        .arg("-cert")
        .arg(ca_cert_file)
        .arg("-gencrl")
        .arg("-out")
        .arg(crl_pem)
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

use crate::{app::AppContext, flows::FlowError, pem::PemCertificate};

pub async fn check_if_we_have_ca_cert(app: &AppContext, ca_cn: &str) -> Result<(), FlowError> {
    let ca_cert_file = app.settings_reader.get_nginx_path().await;

    let file = ca_cert_file.get_ca_cert_file(ca_cn);

    let content = tokio::fs::read(file.as_str()).await;

    if content.is_err() {
        println!("Can not read file: {:?}", file);
        return Err(FlowError::CaNotFound);
    }

    let content = content.unwrap();

    let pem = PemCertificate::from_bytes(content);

    let info = pem.get_cert_info();

    if info.is_err() {
        println!("Can not read CA info from file: {:?}", file);
        return Err(FlowError::CaNotFound);
    }

    let info = info.unwrap();

    if info.0 != ca_cn {
        println!(
            "CA name in file is is {} but we are requesting info from ca: {} from file {}",
            info.0, file, ca_cn
        );
        return Err(FlowError::CaNotFound);
    }

    Ok(())
}

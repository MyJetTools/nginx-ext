use serde::*;

use crate::{
    app::AppContext,
    flows::FlowError,
    pem::{PemCertificate, PemPrivateKey},
    storage::nginx::instance::SslCertsPath,
};

pub async fn upload_certificate(
    app: &AppContext,
    cert: Vec<u8>,
    key: Vec<u8>,
) -> Result<(), FlowError> {
    let cert = PemCertificate::from_bytes(cert);
    let private_key = PemPrivateKey::from_bytes(key);
    {
        let cert = cert.into_certificate();
        let pk = private_key.into_private_key();

        let public_key = cert.public_key().unwrap();

        if !public_key.public_eq(&pk) {
            return Err(FlowError::SomethingWentWrong(
                "Private key does not match the certificate".to_string(),
            ));
        }
    }

    let (domain, expires) = match cert.get_cert_info() {
        Ok(result) => result,
        Err(err) => {
            return Err(FlowError::SomethingWentWrong(err));
        }
    };

    let ssl_cert_path = SslCertsPath::new(&app.settings_reader).await;

    let mut file_name = expires.to_compact_date_time_string();
    file_name.push('.');
    file_name.push_str(domain.replace("*", "%").as_str());

    tokio::fs::write(
        ssl_cert_path.generate_private_key_file(file_name.as_str()),
        private_key.as_slice(),
    )
    .await
    .unwrap();

    tokio::fs::write(
        ssl_cert_path.generate_certificate_file(file_name.as_str()),
        cert.as_slice(),
    )
    .await
    .unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct CertInfo {
    pub domain: String,
    pub expires: String,
}

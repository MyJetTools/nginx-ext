use crate::app::AppContext;

use super::FlowError;

pub async fn get_pfx(
    app: &AppContext,
    ca_cn: &str,
    email: &str,
    password: &str,
) -> Result<Vec<u8>, FlowError> {
    let private_key = crate::storage::cert::load_pem_private_key(app, ca_cn, email).await;

    let private_key = private_key.into_private_key();

    let certificate = crate::storage::cert::load_pem_certificate(app, ca_cn, email).await;

    let certificate = certificate.into_certificate();

    let pkcs12 = openssl::pkcs12::Pkcs12::builder()
        .build(password, email, &private_key, &certificate)
        .unwrap();

    Ok(pkcs12.to_der().unwrap())
}

use openssl::{
    pkey::{PKey, Private},
    x509::X509,
};

use crate::{app::AppContext, my_no_sql::ca_entity::CaMyNoSqlEntity};

use super::FlowError;

pub async fn get_pfx(
    app: &AppContext,
    ca_cn: &str,
    email: &str,
    password: &str,
) -> Result<Vec<u8>, FlowError> {
    let ca = app
        .ca_my_no_sql_writer
        .get_entity(ca_cn, CaMyNoSqlEntity::get_row_key(), None)
        .await
        .unwrap();

    if ca.is_none() {
        return Err(FlowError::CaNotFound);
    }

    let cert: Option<crate::my_no_sql::cert_entity::CertMyNoSqlEntity> = app
        .certs_my_no_sql_writer
        .get_entity(ca_cn, email, None)
        .await
        .unwrap();

    if cert.is_none() {
        return Err(FlowError::CertNotFound);
    }

    let cert = cert.unwrap();

    let client_private_key: PKey<Private> =
        PKey::private_key_from_pem(cert.get_private_key_pem().as_slice()).unwrap();

    let client_cert = X509::from_pem(cert.get_cert_pem().as_slice()).unwrap();

    let pkcs12 = openssl::pkcs12::Pkcs12::builder()
        .build(password, email, &client_private_key, &client_cert)
        .unwrap();

    Ok(pkcs12.to_der().unwrap())
}

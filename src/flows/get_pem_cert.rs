use crate::{app::AppContext, my_no_sql::ca_entity::CaMyNoSqlEntity};

use super::FlowError;

pub async fn get_pem_cert(
    app: &AppContext,
    ca_cn: &str,
    email: &str,
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

    Ok(cert.get_cert_pem())
}

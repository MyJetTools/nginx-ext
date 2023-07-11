use std::sync::Arc;

use openssl::{
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    rsa::Rsa,
    x509::{X509NameBuilder, X509ReqBuilder, X509},
};

use crate::{app::AppContext, my_no_sql::cert_entity::CertMyNoSqlEntity};

pub async fn generate_cert(app: &Arc<AppContext>, ca_cn: &str, email: &str) -> Result<(), String> {
    let ca = app
        .ca_my_no_sql_writer
        .get_entity(ca_cn, crate::my_no_sql::ca_entity::ROW_KEY, None)
        .await
        .unwrap();

    if ca.is_none() {
        return Err(format!("CA with CN {} not found", ca_cn));
    }

    let ca = ca.unwrap();

    let private_key_ca = PKey::private_key_from_pem(&ca.get_private_key()).unwrap();

    // Build the X509 name for the CA
    let mut builder = X509NameBuilder::new().unwrap();
    builder
        .append_entry_by_nid(Nid::COMMONNAME, &ca.partition_key)
        .unwrap();
    builder
        .append_entry_by_text("C", ca.country.as_str())
        .unwrap();

    builder.append_entry_by_text("L", ca.city.as_str()).unwrap();
    builder
        .append_entry_by_text("O", ca.organization.as_str())
        .unwrap();
    let name_ca = builder.build();

    // Generate a 2048 bit RSA private key for the client
    let rsa_client = Rsa::generate(2048).unwrap();
    let pkey_client = PKey::from_rsa(rsa_client).unwrap();

    // Build the X509 name for the client
    let mut builder = X509NameBuilder::new().unwrap();
    builder.append_entry_by_nid(Nid::COMMONNAME, email).unwrap();
    let name_client = builder.build();

    let cert_serial_number = super::get_next_cert_serial_number(app, ca_cn).await;

    // Build the client certificate request
    let mut req_builder = X509ReqBuilder::new().unwrap();
    req_builder.set_version(2).unwrap();
    req_builder.set_subject_name(&name_client).unwrap();
    req_builder.set_pubkey(&pkey_client).unwrap();
    req_builder
        .sign(&pkey_client, MessageDigest::sha256())
        .unwrap();

    // Build the client certificate
    let mut cert_builder = X509::builder().unwrap();
    cert_builder.set_version(2).unwrap();
    let serial_number = openssl::bn::BigNum::from_u32(cert_serial_number)
        .unwrap()
        .to_asn1_integer()
        .unwrap();
    cert_builder.set_serial_number(&serial_number).unwrap();
    cert_builder.set_subject_name(&name_client).unwrap();
    cert_builder.set_issuer_name(&name_ca).unwrap();
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365 * 10).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_pubkey(&pkey_client).unwrap();

    cert_builder
        .sign(&private_key_ca, MessageDigest::sha256())
        .unwrap();
    let cert_client = cert_builder.build();

    let cert_entity = CertMyNoSqlEntity::new(
        ca_cn.to_string(),
        email.to_string(),
        cert_client.to_pem().unwrap(),
        pkey_client.private_key_to_pem_pkcs8().unwrap(),
        cert_serial_number,
    );

    app.certs_my_no_sql_writer
        .insert_entity(&cert_entity)
        .await
        .unwrap();

    Ok(())
}

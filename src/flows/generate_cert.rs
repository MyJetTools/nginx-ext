use std::sync::Arc;

use openssl::{
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    rsa::Rsa,
    x509::{X509NameBuilder, X509ReqBuilder, X509},
};

use crate::app::AppContext;

use super::FlowError;
use crate::storage::utils::*;

pub async fn generate_cert(
    app: &Arc<AppContext>,
    ca_cn: &str,
    email: &str,
) -> Result<(), FlowError> {
    let ca_private_key = crate::storage::ca::load_private_key(app, ca_cn).await;

    //let ca_path = crate::storage::utils::get_ca_path(app, ca_cn).await;
    let path = crate::storage::cert::create_folder_if_not_exists(app, ca_cn, email).await;

    let private_key_file_name = format!("{}/{}", path, PRIVATE_KEY_FILE_NAME);
    let public_key_file_name = format!("{}/{}", path, PUBLIC_KEY_FILE_NAME);
    let cert_file_name = format!("{}/{}", path, CERT_FILE_NAME);

    // Generate a 2048 bit RSA private key for the client
    let rsa_client = Rsa::generate(4096).unwrap();
    let pkey_client = PKey::from_rsa(rsa_client).unwrap();

    tokio::fs::write(
        private_key_file_name,
        pkey_client.private_key_to_pem_pkcs8().unwrap(),
    )
    .await
    .unwrap();

    tokio::fs::write(
        public_key_file_name,
        pkey_client.public_key_to_pem().unwrap(),
    )
    .await
    .unwrap();

    let ca_name = crate::storage::ca::get_509_name(app, ca_cn).await;

    // Build the X509 name for the client
    let mut builder = X509NameBuilder::new().unwrap();
    builder.append_entry_by_nid(Nid::COMMONNAME, email).unwrap();
    let name_client = builder.build();

    let cert_serial_number = crate::storage::ca::get_next_serial_number(app, ca_cn).await;

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
    cert_builder.set_issuer_name(&ca_name).unwrap();
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365 * 10).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_pubkey(&pkey_client).unwrap();

    cert_builder
        .sign(&ca_private_key, MessageDigest::sha256())
        .unwrap();
    let cert_client = cert_builder.build();

    tokio::fs::write(cert_file_name, cert_client.to_pem().unwrap())
        .await
        .unwrap();

    Ok(())
}

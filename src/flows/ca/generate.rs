use std::sync::Arc;

use openssl::{
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    rsa::Rsa,
    x509::{extension::BasicConstraints, X509NameBuilder, X509},
};

use crate::{app::AppContext, pem::PemCertInfo};

pub async fn generate(app: &Arc<AppContext>, cert_info: PemCertInfo) {
    // Generate a 2048 bit RSA private key for the CA
    let rsa_ca = Rsa::generate(4096).unwrap();
    let p_key_ca = PKey::from_rsa(rsa_ca).unwrap();

    // Build the X509 name for the CA
    let mut builder = X509NameBuilder::new().unwrap();
    builder
        .append_entry_by_nid(Nid::COMMONNAME, &cert_info.ca_cn)
        .unwrap();
    builder
        .append_entry_by_text("C", &cert_info.country_code)
        .unwrap();

    builder.append_entry_by_text("L", &cert_info.city).unwrap();
    builder
        .append_entry_by_text("O", &cert_info.organization)
        .unwrap();
    let name_ca = builder.build();

    // Build the CA certificate
    let mut cert_builder = X509::builder().unwrap();
    cert_builder.set_version(2).unwrap();
    let serial_number = openssl::bn::BigNum::from_u32(1)
        .unwrap()
        .to_asn1_integer()
        .unwrap();
    cert_builder.set_serial_number(&serial_number).unwrap();
    cert_builder.set_subject_name(&name_ca).unwrap();
    cert_builder.set_issuer_name(&name_ca).unwrap();
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_pubkey(&p_key_ca).unwrap();

    cert_builder
        .append_extension(BasicConstraints::new().critical().ca().build().unwrap())
        .unwrap();

    cert_builder
        .sign(&p_key_ca, MessageDigest::sha256())
        .unwrap();
    let cert_ca = cert_builder.build();

    crate::storage::ca::write(app, &cert_info, cert_ca.into(), p_key_ca.into()).await
}

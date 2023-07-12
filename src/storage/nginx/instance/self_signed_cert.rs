use openssl::{
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    x509::{extension::SubjectKeyIdentifier, X509},
};

use crate::pem::*;
const CERTS_FOLDER: &str = "/etc/nginx/certs";

pub async fn create_self_signed_ssl_certificate_if_needed() {
    let self_cert_file = format!("{}/self.crt", CERTS_FOLDER);
    let self_pk_file = format!("{}/self.key", CERTS_FOLDER);

    let result_cert = tokio::fs::read(self_cert_file.as_str()).await;
    let result_pk = tokio::fs::read(self_pk_file.as_str()).await;
    if result_cert.is_err() || result_pk.is_err() {
        println!("Self signed cert not found. Generating brand new self signed certificate...");
        let (p_key, cert) = generate_self_signed_ssl_certificate();

        let p_key_content: Vec<u8> = p_key.into();
        tokio::fs::write(self_pk_file, p_key_content.as_slice())
            .await
            .unwrap();

        let cert_content: Vec<u8> = cert.into();
        tokio::fs::write(self_cert_file, cert_content.as_slice())
            .await
            .unwrap();
    }
}

fn generate_self_signed_ssl_certificate() -> (PemPrivateKey, PemCertificate) {
    use openssl::rsa::Rsa;
    use openssl::x509::X509NameBuilder;
    // Generate a new private key.
    let rsa = Rsa::generate(4096).unwrap();
    let p_key = PKey::from_rsa(rsa).unwrap();

    // Build the X509 name.
    let mut x509_name = X509NameBuilder::new().unwrap();
    x509_name
        .append_entry_by_nid(Nid::COMMONNAME, "SelfSigned")
        .unwrap();
    let x509_name = x509_name.build();

    // Build the X509 object.
    let mut x509 = X509::builder().unwrap();
    x509.set_version(2).unwrap();
    x509.set_subject_name(&x509_name).unwrap();
    x509.set_issuer_name(&x509_name).unwrap();
    x509.set_pubkey(&p_key).unwrap();

    let not_after = openssl::asn1::Asn1Time::days_from_now(365 * 10).unwrap();
    x509.set_not_after(&not_after).unwrap();

    // Add the SubjectKeyIdentifier.
    let subject_key_identifier = SubjectKeyIdentifier::new();
    x509.append_extension(
        subject_key_identifier
            .build(&x509.x509v3_context(None, None))
            .unwrap(),
    )
    .unwrap();

    // Sign the certificate with the private key.
    x509.sign(&p_key, MessageDigest::sha256()).unwrap();

    // Get the certificate
    let cert = x509.build();

    (p_key.into(), cert.into())
}

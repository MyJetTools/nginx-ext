use crate::{app::AppContext, pem::*};

pub const SELF_SIGNED_CERT_NAME: &str = "self";

pub async fn create_self_signed_ssl_certificate_if_needed(app: &AppContext) {
    let nginx_path = app.settings_reader.get_nginx_path().await;

    let cert_file_name = nginx_path.get_self_signed_cert_file_name();
    let private_key_file_name = nginx_path.get_self_signed_private_key_file_name();

    let result_cert = tokio::fs::read(cert_file_name.as_str()).await;
    let result_pk = tokio::fs::read(private_key_file_name.as_str()).await;
    if result_cert.is_err() || result_pk.is_err() {
        println!("Self signed cert not found. Generating brand new self signed certificate...");
        let (p_key, cert) = generate_self_signed_ssl_certificate();

        let p_key_content: Vec<u8> = p_key.into();
        tokio::fs::write(private_key_file_name, p_key_content.as_slice())
            .await
            .unwrap();

        let cert_content: Vec<u8> = cert.into();
        tokio::fs::write(cert_file_name, cert_content.as_slice())
            .await
            .unwrap();
    }
}

fn generate_self_signed_ssl_certificate() -> (PemPrivateKey, PemCertificate) {
    use rcgen::generate_simple_self_signed;
    let subject_alt_names = vec!["localhost".into(), "127.0.0.1".into()];
    let cert = generate_simple_self_signed(subject_alt_names).unwrap();

    let key = cert.serialize_private_key_pem().into_bytes();
    let cert = cert.serialize_pem().unwrap().into_bytes();

    (
        PemPrivateKey::from_bytes(key),
        PemCertificate::from_bytes(cert),
    )
}

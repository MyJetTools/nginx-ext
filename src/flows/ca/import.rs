use crate::{
    app::AppContext,
    pem::{PemCertificate, PemPrivateKey},
};

pub async fn import(app: &AppContext, ca_cn: &str, private_key: Vec<u8>, cert: Vec<u8>) {
    let cert_ca = PemCertificate::from_bytes(cert);

    let private_key = PemPrivateKey::from_bytes(private_key);

    let mut cert_info = cert_ca.get_pem_info();

    cert_info.ca_cn = ca_cn.to_string();

    crate::storage::ca::write(app, &cert_info, cert_ca, private_key).await
}

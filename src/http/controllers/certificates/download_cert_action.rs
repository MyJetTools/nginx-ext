use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_swagger::MyHttpInput;
use openssl::{
    pkey::{PKey, Private},
    x509::X509,
};

use crate::{app::AppContext, my_no_sql::ca_entity::ROW_KEY};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/certificates/v1/downloadCert",
    summary: "Download pfx",
    description: "Download pfx",
    controller: "Certificates",
    input_data: "DownloadClientCertInputModel",
    result:[
        {status_code: 200, description: "Certificate as a text"},
    ]
)]
pub struct DownloadCertAction {
    app: Arc<AppContext>,
}

impl DownloadCertAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadCertAction,
    input_data: DownloadClientCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let ca = action
        .app
        .ca_my_no_sql_writer
        .get_entity(&input_data.ca_name, ROW_KEY, None)
        .await
        .unwrap();

    if ca.is_none() {
        return Err(HttpFailResult::as_not_found(
            "CA not found".to_string(),
            false,
        ));
    }

    let ca = ca.unwrap();

    let cert: Option<crate::my_no_sql::cert_entity::CertMyNoSqlEntity> = action
        .app
        .certs_my_no_sql_writer
        .get_entity(&input_data.ca_name, &input_data.email, None)
        .await
        .unwrap();

    if cert.is_none() {
        return Err(HttpFailResult::as_not_found(
            "Certificate not found".to_string(),
            false,
        ));
    }

    let cert = cert.unwrap();

    let client_private_key: PKey<Private> =
        PKey::private_key_from_pem(cert.get_private_key_pem().as_slice()).unwrap();

    let client_cert = X509::from_pem(cert.get_cert_pem().as_slice()).unwrap();

    let pkcs12 = openssl::pkcs12::Pkcs12::builder()
        .build(
            &input_data.password,
            &input_data.email,
            &client_private_key,
            &client_cert,
        )
        .unwrap();

    // The DER-encoded bytes of the archive

    return HttpOutput::as_file("cert.pfx".to_string(), pkcs12.to_der().unwrap())
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadClientCertInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
    #[http_query(name = "email", description = "Email")]
    pub email: String,
    #[http_query(name = "password", description = "Certificate Password")]
    pub password: String,
}

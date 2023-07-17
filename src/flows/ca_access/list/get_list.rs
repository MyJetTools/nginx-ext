use crate::{app::AppContext, flows::FlowError};

pub async fn get_list(app: &AppContext, ca_cn: &str) -> Result<Vec<String>, FlowError> {
    let read_access = app.config_file_content.read().await;

    if read_access.client_cert_accesses.is_none() {
        return Err(FlowError::CaNotFound);
    }

    let client_cert_list = read_access.client_cert_accesses.as_ref().unwrap();

    if let Some(client_cert_list) = client_cert_list.get(ca_cn) {
        return Ok(client_cert_list.keys().map(|itm| itm.clone()).collect());
    }

    Err(FlowError::CaNotFound)
}

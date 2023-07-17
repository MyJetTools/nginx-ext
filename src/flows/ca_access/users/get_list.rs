use crate::{app::AppContext, flows::FlowError};

pub async fn get_list(
    app: &AppContext,
    ca_cn: &str,
    access_list_id: &str,
) -> Result<Vec<String>, FlowError> {
    let read_access = app.config_file_content.write().await;

    if read_access.client_cert_accesses.is_none() {
        return Err(FlowError::CaNotFound);
    }

    let by_ca = read_access.client_cert_accesses.as_ref().unwrap();

    if let Some(list_by_ca) = by_ca.get(ca_cn) {
        if let Some(access_list) = list_by_ca.get(access_list_id) {
            let mut result = Vec::new();
            for itm in access_list.clone() {
                result.push(itm);
            }

            return Ok(result);
        } else {
            return Err(FlowError::ValidationError(
                "Access list not found".to_string(),
            ));
        }
    }

    Err(FlowError::CaNotFound)
}

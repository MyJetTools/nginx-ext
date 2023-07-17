use crate::{app::AppContext, flows::FlowError};

pub async fn add_user(
    app: &AppContext,
    ca_cn: &str,
    access_list_id: &str,
    cert_cn: &str,
) -> Result<(), FlowError> {
    let mut write_access = app.config_file_content.write().await;

    if write_access.client_cert_accesses.is_none() {
        return Err(FlowError::CaNotFound);
    }

    let by_ca = write_access.client_cert_accesses.as_mut().unwrap();

    if let Some(list_by_ca) = by_ca.get_mut(ca_cn) {
        if let Some(access_list) = list_by_ca.get_mut(access_list_id) {
            for itm in access_list.clone() {
                if itm == cert_cn {
                    return Err(FlowError::ValidationError(
                        "User already exists".to_string(),
                    ));
                }
            }

            access_list.push(cert_cn.to_string());
            crate::storage::model::save(&app.settings_reader, &write_access).await;

            return Ok(());
        } else {
            return Err(FlowError::ValidationError(
                "Access list not found".to_string(),
            ));
        }
    }

    Err(FlowError::CaNotFound)
}

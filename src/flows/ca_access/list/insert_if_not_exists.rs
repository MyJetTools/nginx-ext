use crate::{app::AppContext, flows::FlowError};

pub async fn insert_if_not_exists(
    app: &AppContext,
    ca_name: &str,
    access_list_id: &str,
) -> Result<(), FlowError> {
    let mut write_access = app.config_file_content.write().await;

    if write_access.has_config_file_content(ca_name, access_list_id) {
        return Err(FlowError::ValidationError(
            "Access list already exists".to_string(),
        ));
    }

    write_access.insert_access_list(ca_name, access_list_id);

    crate::storage::model::save(&app.settings_reader, &write_access).await;

    Ok(())
}

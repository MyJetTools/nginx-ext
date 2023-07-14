use crate::{app::AppContext, flows::FlowError, storage::nginx::models::HttpConfig};

pub async fn insert_or_replace(
    app: &AppContext,
    name: String,
    config: HttpConfig,
) -> Result<(), FlowError> {
    if let Some(ca_cn) = config.ca_cn.as_ref() {
        crate::flows::ca::check_if_we_have_ca_cert(app, ca_cn).await?;
    }

    crate::storage::nginx::http_configs::insert_or_replace(&app, name, config).await;
    crate::flows::generate_nginx_config_and_reload_nginx(&app).await?;

    Ok(())
}

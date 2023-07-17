use std::sync::Arc;

use rust_extensions::AppStates;
use tokio::sync::RwLock;

use crate::{settings::SettingsReader, storage::model::NginxFileContent};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings_reader: Arc<SettingsReader>,
    pub nginx_file_content: RwLock<NginxFileContent>,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        let nginx_file_content = crate::storage::nginx::load(&settings_reader).await;
        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            settings_reader,
            nginx_file_content: RwLock::new(nginx_file_content),
        }
    }
}

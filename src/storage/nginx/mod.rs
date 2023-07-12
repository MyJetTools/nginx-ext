mod load;
pub mod models;
pub use load::*;
mod save;
pub use save::*;
pub mod instance;
pub mod up_streams;

use crate::settings::SettingsReader;

pub async fn get_nginx_file_name(settings_reader: &SettingsReader) -> String {
    let mut file_name = settings_reader.get_data_path().await;
    file_name.push_str("nginx.cfg");

    file_name
}

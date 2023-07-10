use std::sync::Arc;

use app::AppContext;

mod app;
mod flows;
mod http;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".ca_api").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(settings_reader));

    crate::http::start(&app);

    app.app_states.wait_until_shutdown().await;
}

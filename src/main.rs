use std::sync::Arc;

use app::AppContext;

mod app;
mod base_64;

mod flows;
mod http;
mod pem;
mod settings;
mod storage;
mod to_hex;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".ca_api").await;
    let settings_reader = Arc::new(settings_reader);

    println!("Starting nginx");

    let output = tokio::process::Command::new("nginx")
        .output()
        .await
        .unwrap();

    println!("Nginx start result: {:?}", output);

    let app = Arc::new(AppContext::new(settings_reader).await);

    crate::http::start(&app);

    app.app_states.wait_until_shutdown().await;
}

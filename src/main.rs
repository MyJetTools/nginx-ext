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
    let settings_reader = settings::SettingsReader::new(".nginx-ext").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(settings_reader).await);

    if app.settings_reader.get_start_nginx().await {
        {
            crate::storage::nginx::instance::write_nginx_conf().await;

            let content = app.nginx_file_content.read().await;
            crate::storage::nginx::instance::generate_config_file(&app, &content).await;
        }

        println!("Starting nginx");

        let output = tokio::process::Command::new("nginx")
            .output()
            .await
            .unwrap();

        println!("Nginx start result: {:?}", output);
    }

    crate::http::start(&app);

    app.app_states.wait_until_shutdown().await;
}

use std::sync::Arc;

use app::AppContext;

mod app;
mod base_64;

mod flows;
mod http;
mod pem;
mod settings;
mod ssl_certificates;
mod storage;
mod to_hex;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".nginx-ext").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(settings_reader).await);

    crate::storage::nginx::instance::create_self_signed_ssl_certificate_if_needed(&app).await;

    if app.settings_reader.get_start_nginx().await {
        crate::storage::nginx::instance::write_nginx_conf().await;
        crate::storage::nginx::instance::write_default_conf().await;

        let ssl_certs = crate::flows::ssl::get_list_of_certificates(&app).await;

        {
            let content = app.nginx_file_content.read().await;
            crate::storage::nginx::instance::generate_config_file(&app, &content, &ssl_certs).await;
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

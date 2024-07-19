use std::sync::Arc;

use app::AppContext;

mod app;

mod config_path;
mod flows;
mod http;
mod pem;
mod settings;
mod ssl_certificates;
mod storage;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".nginx-ext").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(settings_reader).await);

    init_file_system(&app).await;

    if app.settings_reader.get_start_nginx().await {
        tokio::spawn(async move {
            println!("Starting nginx");

            let output = tokio::process::Command::new("nginx")
                .output()
                .await
                .unwrap();

            println!("Nginx start result: {:?}", output);
        });
    }

    crate::http::start(&app);

    app.app_states.wait_until_shutdown().await;
}

async fn init_file_system(app: &AppContext) {
    let nginx_path = app.settings_reader.get_nginx_path().await;
    crate::storage::nginx::instance::write_nginx_conf(&nginx_path).await;
    crate::storage::nginx::instance::write_default_conf(&nginx_path).await;

    let ssl_certs = crate::flows::ssl::get_list_of_certificates(&app).await;

    {
        let content = app.config_file_content.read().await;
        crate::storage::nginx::instance::generate_config_file(&content, &ssl_certs, &nginx_path)
            .await;
    }

    let config_path = app.settings_reader.get_config_path().await;
    tokio::fs::create_dir_all(config_path.into_ca_path())
        .await
        .unwrap();

    crate::storage::nginx::instance::create_self_signed_ssl_certificate_if_needed(&app).await;
}

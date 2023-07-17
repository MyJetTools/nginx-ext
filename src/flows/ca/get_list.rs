use crate::app::AppContext;

pub async fn get_list(app: &AppContext) -> Vec<String> {
    let cert_path = app.settings_reader.get_nginx_path().await.get_certs_path();

    let mut read_handle = tokio::fs::read_dir(cert_path.as_str()).await.unwrap();

    let mut result = Vec::new();

    while let Some(file) = read_handle.next_entry().await.unwrap() {
        let file_name = file.file_name().into_string().unwrap();

        if !file_name.ends_with(".crt") {
            continue;
        }

        if !file_name.starts_with("ca_") {
            continue;
        }

        result.push(file_name[3..file_name.len() - 4].to_string());
    }

    result
}

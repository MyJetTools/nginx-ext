use crate::storage::nginx::models::NginxFileContent;

pub async fn generate_up_streams_file(content: &NginxFileContent) {
    let file = super::get_up_streams_file_name();

    if content.upstreams.len() == 0 {
        tokio::fs::remove_file(file).await.unwrap();
    } else {
        let content = content.generate_nginx_up_streams_configuration();
        tokio::fs::write(file, content).await.unwrap();
    }
}

use super::NginxPath;

pub async fn write_nginx_conf(nginx_path: &NginxPath) {
    let content = generate_nginx_conf_content(nginx_path);
    tokio::fs::write(nginx_path.get_config_file(), content)
        .await
        .unwrap();
}

pub fn generate_nginx_conf_content(nginx_path: &NginxPath) -> String {
    let the_nginx_path_str = nginx_path.as_str();

    let path_to_generate_http_content = nginx_path.get_path_to_generate_http_content();

    let path_to_generate_tcp_content = nginx_path.get_path_to_generate_tcp_content();

    format!(
        r#"
    user  nginx;
worker_processes  auto;

error_log  /var/log/nginx/error.log notice;
pid        /var/run/nginx.pid;


events {{
    worker_connections  1024;
}}


http {{
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
                      '$status $body_bytes_sent "$http_referer" '
                      '"$http_user_agent" "$http_x_forwarded_for"';

    access_log  /var/log/nginx/access.log  main;

    sendfile        on;
    #tcp_nopush     on;

    keepalive_timeout  65;

    #gzip  on;

    include {the_nginx_path_str}conf.d/*.conf;
    include {path_to_generate_http_content}*.conf;
}}

stream {{
    include {path_to_generate_tcp_content}*.stream;
}}
    "#
    )
}

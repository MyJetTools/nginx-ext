use super::NginxPath;

pub async fn write_nginx_conf(nginx_path: &NginxPath) {
    let content = generate_nginx_conf_content();
    tokio::fs::write(nginx_path.get_config_file(), content)
        .await
        .unwrap();
}

pub fn generate_nginx_conf_content() -> &'static str {
    r#"
    user  nginx;
worker_processes  auto;

error_log  /var/log/nginx/error.log notice;
pid        /var/run/nginx.pid;


events {
    worker_connections  1024;
}


http {
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

    include /etc/nginx/conf.d/*.conf;
    include /etc/nginx/sites-enabled/*.conf;
}

stream {
    include /etc/nginx/streams/*.stream;
}
    "#
}

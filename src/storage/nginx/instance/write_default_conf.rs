pub async fn write_default_conf() {
    let file = "/etc/nginx/sites-enabled/default-site.conf";

    let content = get_default_conf_content();
    tokio::fs::write(file, content).await.unwrap();
}

pub fn get_default_conf_content() -> &'static str {
    r#"client_max_body_size 50M;
    fastcgi_read_timeout 300;
    proxy_connect_timeout       300;
    proxy_send_timeout          300;
    proxy_read_timeout          300;
    send_timeout                300;
    
        server {
    
            listen 80 default;
    
            access_log off;
            error_log off;
    
            return 301 https://$host$request_uri;
    
        }"#
}

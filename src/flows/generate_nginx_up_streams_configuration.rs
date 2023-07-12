use crate::app::AppContext;

pub async fn generate_nginx_up_streams_configuration(app: &AppContext) -> String {
    let nginx_content = app.nginx_file_content.read().await;

    let mut result = String::new();

    for (name, routes) in &nginx_content.upstreams {
        result.push_str("upstream ");
        result.push_str(name);
        result.push_str(" {\n");

        for rt in routes {
            result.push_str("  server ");
            result.push_str(rt.remote_addr.as_str());

            if let Some(w) = rt.weight {
                result.push_str(" weight=");
                result.push_str(w.to_string().as_str());
            }

            if rt.is_backup {
                result.push_str(" backup");
            }

            result.push_str(";\n")
        }

        result.push_str("}\n\n");
    }

    result
}

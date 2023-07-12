mod generate_up_streams_file;
pub use generate_up_streams_file::*;

const ROOT_NGINX_UP_STREAMS_PATH: &str = "/etc/nginx/sites-enabled/";

pub fn get_up_streams_file_name() -> String {
    let mut path = String::from(ROOT_NGINX_UP_STREAMS_PATH);
    path.push_str("gen_upstreams.conf");
    path
}

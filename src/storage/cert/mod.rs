mod create_folder_if_not_exists;
mod write;
pub use create_folder_if_not_exists::*;
pub use write::*;

use crate::app::AppContext;

pub async fn compile_cert_path(app: &AppContext, ca_cn: &str, email: &str) -> String {
    let mut path = super::utils::get_ca_path(app, ca_cn).await;

    let sub_path = email.find("@").unwrap();

    let sub_path = &email[..sub_path];

    path.push_str("/certs/");
    path.push_str(sub_path);

    path
}

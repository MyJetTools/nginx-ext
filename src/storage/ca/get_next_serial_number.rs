use crate::{
    app::AppContext,
    to_hex::{FromHex, ToHex},
};

pub async fn get_next_serial_number(app: &AppContext, ca_cn: &str) -> u32 {
    let file = super::super::utils::get_serial_file_name(app, ca_cn).await;

    let content = tokio::fs::read_to_string(file.as_str()).await.unwrap();

    let mut value = content.as_str().from_hex();

    value += 1;

    tokio::fs::write(file.as_str(), value.to_hex())
        .await
        .unwrap();

    value
}

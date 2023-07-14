use crate::{
    app::AppContext,
    to_hex::{FromHex, ToHex},
};

pub async fn get_next_serial_number(app: &AppContext, ca_cn: &str) -> u32 {
    let serial_file_name = app
        .settings_reader
        .get_ca_data_path(ca_cn.into())
        .await
        .into_serial_file_name();

    let content = tokio::fs::read_to_string(serial_file_name.as_str())
        .await
        .unwrap();

    let mut value = content.as_str().from_hex();

    value += 1;

    tokio::fs::write(serial_file_name, value.to_hex())
        .await
        .unwrap();

    value
}

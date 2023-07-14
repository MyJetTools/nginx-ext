use openssl::x509::X509Name;
use rust_extensions::slice_of_u8_utils::SliceOfU8Ext;

use crate::app::AppContext;

pub async fn get_509_name(app: &AppContext, cn_name: &str) -> X509Name {
    let ca_path = app.settings_reader.get_ca_data_path(cn_name.into()).await;

    let content = tokio::fs::read_to_string(ca_path.to_config_file_name())
        .await
        .unwrap();

    let country = extract_value(content.as_str(), "countryName");
    let city = extract_value(content.as_str(), "localityName");
    let org = extract_value(content.as_str(), "organizationName");

    let mut builder = openssl::x509::X509NameBuilder::new().unwrap();
    builder
        .append_entry_by_nid(openssl::nid::Nid::COMMONNAME, cn_name)
        .unwrap();
    builder.append_entry_by_text("C", country).unwrap();

    builder.append_entry_by_text("L", city).unwrap();
    builder.append_entry_by_text("O", org).unwrap();
    builder.build()
}

fn extract_value<'s>(src: &'s str, key: &str) -> &'s str {
    let src: &[u8] = src.as_bytes();
    let index = src.find_sequence_pos(key.as_bytes(), 0);
    if index.is_none() {
        panic!("Key {} not found", key);
    }

    let index = index.unwrap();

    let eq = src.find_sequence_pos(b"=", index + 1).unwrap();

    let non_space = src.find_pos_by_condition(eq + 1, |b| b > 32).unwrap();

    let space = src.find_pos_by_condition(non_space + 1, |b| b <= 32);

    match space {
        Some(space) => std::str::from_utf8(&src[non_space..space]).unwrap(),
        None => std::str::from_utf8(&src[non_space..]).unwrap(),
    }
}

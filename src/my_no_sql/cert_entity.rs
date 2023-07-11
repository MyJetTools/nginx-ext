use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("cert")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CertMyNoSqlEntity {
    #[serde(rename = "certBase64")]
    pub cert: String,
    #[serde(rename = "privKeyBase64")]
    pub private_key: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: u32,
}

impl CertMyNoSqlEntity {
    pub fn new(
        cn: String,
        email: String,
        cert: Vec<u8>,
        private_key: Vec<u8>,
        serial_number: u32,
    ) -> Self {
        Self {
            partition_key: cn,
            row_key: email,
            time_stamp: "".to_string(),
            cert: base64::encode(cert),
            private_key: base64::encode(private_key),
            serial_number,
        }
    }

    pub fn get_cert_pem(&self) -> Vec<u8> {
        base64::decode(&self.cert).unwrap()
    }

    pub fn get_private_key_pem(&self) -> Vec<u8> {
        base64::decode(&self.private_key).unwrap()
    }
}

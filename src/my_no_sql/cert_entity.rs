use serde::{Deserialize, Serialize};

use crate::base_64::*;

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
            cert: cert.to_base64(),
            private_key: private_key.to_base64(),
            serial_number,
        }
    }

    pub fn get_cert_pem(&self) -> Vec<u8> {
        self.cert.as_str().from_base64()
    }

    pub fn get_private_key_pem(&self) -> Vec<u8> {
        self.private_key.as_str().from_base64()
    }
}

use serde::{Deserialize, Serialize};

pub const ROW_KEY: &str = "ca";

#[my_no_sql_macros::my_no_sql_entity("cas")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaMyNoSqlEntity {
    pub country: String,
    pub city: String,
    pub organization: String,
    #[serde(rename = "pubKeyBase64")]
    pub public_key: String,
    #[serde(rename = "privKeyBase64")]
    pub private_key: String,
    #[serde(rename = "caCertBase64")]
    pub ca_cert: String,
}

impl CaMyNoSqlEntity {
    pub fn new(
        cn: String,
        country: String,
        city: String,
        organization: String,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        ca_cert: Vec<u8>,
    ) -> Self {
        Self {
            partition_key: cn,
            row_key: ROW_KEY.to_string(),
            time_stamp: "".to_string(),
            country,
            city,
            organization,
            public_key: base64::encode(public_key),
            private_key: base64::encode(private_key),
            ca_cert: base64::encode(ca_cert),
        }
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        base64::decode(&self.public_key).unwrap()
    }

    pub fn get_private_key(&self) -> Vec<u8> {
        base64::decode(&self.private_key).unwrap()
    }

    pub fn get_ca_cert(&self) -> Vec<u8> {
        base64::decode(&self.ca_cert).unwrap()
    }
}

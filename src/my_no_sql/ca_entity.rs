use openssl::x509::X509Name;
use serde::{Deserialize, Serialize};

use crate::base_64::{FromBase64, ToBase64};

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
    pub fn get_row_key() -> &'static str {
        "ca"
    }

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
            row_key: Self::get_row_key().to_string(),
            time_stamp: "".to_string(),
            country,
            city,
            organization,
            public_key: public_key.to_base64(),
            private_key: private_key.to_base64(),
            ca_cert: ca_cert.to_base64(),
        }
    }

    /*
       pub fn get_public_key(&self) -> Vec<u8> {
           self.public_key.as_str().from_base64()
       }
    */
    pub fn get_private_key_content(&self) -> Vec<u8> {
        self.private_key.as_str().from_base64()
    }

    pub fn get_private_key(&self) -> openssl::pkey::PKey<openssl::pkey::Private> {
        openssl::pkey::PKey::private_key_from_pem(self.get_private_key_content().as_slice())
            .unwrap()
    }

    pub fn get_ca_cert_content(&self) -> Vec<u8> {
        self.ca_cert.as_str().from_base64()
    }

    pub fn get_ca_cert(&self) -> openssl::x509::X509 {
        openssl::x509::X509::from_pem(self.get_ca_cert_content().as_slice()).unwrap()
    }

    pub fn get_x509_name(&self) -> X509Name {
        // Build the X509 name for the CA
        let mut builder = openssl::x509::X509NameBuilder::new().unwrap();
        builder
            .append_entry_by_nid(openssl::nid::Nid::COMMONNAME, &self.partition_key)
            .unwrap();
        builder
            .append_entry_by_text("C", self.country.as_str())
            .unwrap();

        builder
            .append_entry_by_text("L", self.city.as_str())
            .unwrap();
        builder
            .append_entry_by_text("O", self.organization.as_str())
            .unwrap();
        builder.build()
    }
}

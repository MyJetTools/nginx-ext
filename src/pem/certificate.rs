use openssl::x509::X509;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct PemCertificate(Vec<u8>);

impl PemCertificate {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    pub fn into_certificate(&self) -> X509 {
        X509::from_pem(self.0.as_slice()).unwrap()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn get_cert_info(&self) -> Result<(String, DateTimeAsMicroseconds), String> {
        let x_509: X509 = self.into_certificate();

        let subject_name = x_509.subject_name();

        let common_name = subject_name.entries_by_nid(openssl::nid::Nid::COMMONNAME);

        for entry in common_name {
            let not_after = x_509.not_after();

            let not_after = not_after.to_string();

            let dt = DateTimeAsMicroseconds::from_str(not_after.as_str());

            if dt.is_none() {
                return Err(format!(
                    "Invalid expiration date format inside the certificate: {}",
                    not_after
                ));
            }

            return Ok((
                format!("{}", entry.data().as_utf8().unwrap().trim().to_string()),
                dt.unwrap(),
            ));
        }

        panic!("Common name not found. Probably Certificate is invalid");
    }
}

impl<'s> Into<X509> for &'s PemCertificate {
    fn into(self) -> X509 {
        self.into_certificate()
    }
}

impl Into<Vec<u8>> for PemCertificate {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl Into<String> for PemCertificate {
    fn into(self) -> String {
        String::from_utf8(self.0).unwrap()
    }
}

impl Into<PemCertificate> for X509 {
    fn into(self) -> PemCertificate {
        PemCertificate::from_bytes(self.to_pem().unwrap())
    }
}

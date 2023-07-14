use openssl::x509::X509;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::PemCertInfo;

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

    pub fn get_pem_info(&self) -> PemCertInfo {
        let x_509: X509 = self.into_certificate();

        let subject_name = x_509.subject_name();

        let common_name = subject_name.entries_by_nid(openssl::nid::Nid::COMMONNAME);

        let mut result = PemCertInfo {
            ca_cn: "".to_string(),
            organization: "".to_string(),
            country_code: "".to_string(),
            city: "".to_string(),
        };
        for entry in common_name {
            result.ca_cn = entry.data().as_utf8().unwrap().trim().to_string();
        }

        for entry in subject_name.entries() {
            match entry.object().to_string().as_str() {
                "organizationName" => {
                    result.organization = entry.data().as_utf8().unwrap().trim().to_string();
                }
                "localityName" => {
                    result.city = entry.data().as_utf8().unwrap().trim().to_string();
                }
                "countryName" => {
                    result.country_code = entry.data().as_utf8().unwrap().trim().to_string();
                }
                _ => {}
            }
        }

        result
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

#[cfg(test)]
mod tests {
    use super::PemCertificate;

    #[test]
    fn test() {
        let ca_cert = r#"-----BEGIN CERTIFICATE-----
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
eAVqF+FMrwAJt6+8Gbb7UGZJNwMh7ko0kg==
-----END CERTIFICATE-----
        "#;

        let cert = PemCertificate::from_bytes(ca_cert.as_bytes().to_vec());

        let cert_info = cert.get_pem_info();

        println!("cert_info: {:#?}", cert_info);
    }
}

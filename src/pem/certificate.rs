use openssl::x509::X509;

pub struct PemCertificate(Vec<u8>);

impl PemCertificate {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    pub fn into_certificate(&self) -> X509 {
        X509::from_pem(self.0.as_slice()).unwrap()
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

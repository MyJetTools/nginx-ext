use openssl::pkey::{PKey, Private};

pub struct PemPrivateKey(Vec<u8>);

impl PemPrivateKey {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn into_private_key(&self) -> PKey<Private> {
        PKey::private_key_from_pem(self.0.as_slice()).unwrap()
    }
}

impl<'s> Into<PKey<Private>> for &'s PemPrivateKey {
    fn into(self) -> PKey<Private> {
        self.into_private_key()
    }
}

impl Into<Vec<u8>> for PemPrivateKey {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl Into<String> for PemPrivateKey {
    fn into(self) -> String {
        String::from_utf8(self.0).unwrap()
    }
}
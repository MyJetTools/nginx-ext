pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Vec<u8> {
    fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(self)
    }
}

pub trait FromBase64 {
    fn from_base64(&self) -> Vec<u8>;
}

impl<'s> FromBase64 for &'s str {
    fn from_base64(&self) -> Vec<u8> {
        use base64::Engine;
        match base64::engine::general_purpose::STANDARD.decode(self.as_bytes()) {
            Ok(data) => data,
            Err(e) => panic!("Error decoding base64: {}", e),
        }
    }
}

impl<'s> FromBase64 for &'s String {
    fn from_base64(&self) -> Vec<u8> {
        use base64::Engine;
        match base64::engine::general_purpose::STANDARD.decode(self.as_bytes()) {
            Ok(data) => data,
            Err(e) => panic!("Error decoding base64: {}", e),
        }
    }
}

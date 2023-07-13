use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec};

pub struct SslCertificate {
    pub domain: String,
    pub expires_at: DateTimeAsMicroseconds,
    pub file_name: String,
}

impl SslCertificate {
    pub fn is_wild_cards(&self) -> bool {
        self.domain.starts_with("*")
    }
    pub fn is_my_domain(&self, domain: &str, now: DateTimeAsMicroseconds) -> bool {
        if self.domain == domain && self.expires_at.unix_microseconds > now.unix_microseconds {
            return true;
        }

        if self.is_wild_cards() {
            let index = self.domain.find('.');
            if index.is_none() {
                return false;
            }

            let the_domain = &self.domain[index.unwrap() + 1..];

            if the_domain == domain && self.expires_at.unix_microseconds > now.unix_microseconds {
                return true;
            }
        }

        false
    }
}

pub struct SslCertificates {
    values: Vec<SslCertificate>,
}

impl SslCertificates {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: SslCertificate) {
        self.values.push(value);
    }

    pub fn into_inner(self) -> Vec<SslCertificate> {
        self.values
    }

    pub fn get_by_domain(
        &self,
        domain: &str,
        now: DateTimeAsMicroseconds,
    ) -> Option<&SslCertificate> {
        let mut result = LazyVec::new();
        for itm in &self.values {
            if itm.is_my_domain(domain, now) {
                result.add(itm)
            }
        }

        let mut result = result.get_result()?;

        for itm in &self.values {
            if itm.is_wild_cards() {
                return Some(itm);
            }
        }

        Some(result.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::SslCertificates;

    #[test]
    fn test_domain_with_wild_card() {
        let mut certs_repo = SslCertificates::new();

        certs_repo.push(super::SslCertificate {
            domain: "*.test.com".to_string(),
            expires_at: DateTimeAsMicroseconds::from_str("20220101000000").unwrap(),
            file_name: "20220101000000.test.com".to_string(),
        });

        let found_cert = certs_repo.get_by_domain(
            "test.com",
            DateTimeAsMicroseconds::from_str("20210101000000").unwrap(),
        );

        assert!(found_cert.is_some());
    }

    #[test]
    fn test_domain() {
        let mut certs_repo = SslCertificates::new();

        certs_repo.push(super::SslCertificate {
            domain: "test.com".to_string(),
            expires_at: DateTimeAsMicroseconds::from_str("20220101000000").unwrap(),
            file_name: "20220101000000.test.com".to_string(),
        });

        let found_cert = certs_repo.get_by_domain(
            "test.com",
            DateTimeAsMicroseconds::from_str("20210101000000").unwrap(),
        );

        assert!(found_cert.is_some());
    }
}

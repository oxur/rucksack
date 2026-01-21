use serde::{Deserialize, Serialize};
use url::Url;

use crate::records::DecryptedRecord;

use super::firefox;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Record {
    pub fn to_decrypted(&self) -> DecryptedRecord {
        let ffr = firefox::new_with_password(
            self.url.clone(),
            self.username.clone(),
            self.password.clone(),
        );
        ffr.to_decrypted()
    }
}

pub fn from_decrypted(r: DecryptedRecord) -> Record {
    let url = r.metadata().url;
    let parsed = Url::parse(&url).unwrap();
    Record {
        name: parsed.host_str().unwrap().to_string(),
        url,
        username: r.user(),
        password: r.password(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records;

    #[test]
    fn test_record_to_decrypted() {
        let chrome_record = Record {
            name: "example.com".to_string(),
            url: "https://example.com".to_string(),
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        };

        let decrypted = chrome_record.to_decrypted();
        assert_eq!(decrypted.secrets.user, "testuser");
        assert_eq!(decrypted.secrets.password, "testpass");
        assert_eq!(decrypted.metadata.url, "https://example.com");
    }

    #[test]
    fn test_from_decrypted() {
        let mut decrypted = records::DecryptedRecord {
            secrets: records::secrets_from_user_pass("testuser", "testpass"),
            metadata: records::default_metadata(),
            history: Vec::new(),
        };
        decrypted.metadata.url = "https://example.com/login".to_string();

        let chrome_record = from_decrypted(decrypted);
        assert_eq!(chrome_record.name, "example.com");
        assert_eq!(chrome_record.url, "https://example.com/login");
        assert_eq!(chrome_record.username, "testuser");
        assert_eq!(chrome_record.password, "testpass");
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = Record {
            name: "test.com".to_string(),
            url: "https://test.com".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        let decrypted = original.to_decrypted();
        let converted_back = from_decrypted(decrypted);

        assert_eq!(converted_back.username, "user");
        assert_eq!(converted_back.password, "pass");
        assert_eq!(converted_back.url, "https://test.com");
    }
}

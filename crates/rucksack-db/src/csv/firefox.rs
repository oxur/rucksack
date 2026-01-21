use serde::{Deserialize, Serialize};
use uuid::Uuid;

use rucksack_lib::time;

use crate::records;
use crate::records::secrets_from_user_pass;

// This started as the Firefox login data struct, but it has more fields than
// others, so it has become the default interim struct to which others convert
// for imports.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub url: String,
    pub username: String,
    pub password: String,
    pub http_realm: String,
    pub form_action_origin: String,
    pub guid: String,
    pub time_created: i64,
    pub time_last_used: i64,
    pub time_password_changed: i64,
}

pub fn new(url: String, username: String) -> Record {
    new_with_password(url, username, "".to_string())
}

pub fn new_with_password(url: String, username: String, password: String) -> Record {
    Record {
        url,
        username,
        password,

        ..Default::default()
    }
}

impl Record {
    pub fn to_decrypted(&self) -> records::DecryptedRecord {
        let secrets = secrets_from_user_pass(self.username.as_str(), self.password.as_str());
        let mut metadata = records::default_metadata();
        metadata.name = secrets.user.clone();
        metadata.url = self.url.clone();
        metadata.created = time::epoch_to_string(self.time_created);
        metadata.password_changed = time::epoch_to_string(self.time_password_changed);
        metadata.last_used = time::epoch_to_string(self.time_last_used);
        records::DecryptedRecord {
            secrets,
            metadata,
            history: Vec::<records::History>::new(),
        }
    }
}

pub fn from_decrypted(dr: records::DecryptedRecord) -> Record {
    let md = dr.metadata();
    let mut name = md.name.clone();
    if name.is_empty() {
        name = dr.secrets.user.clone();
    };
    Record {
        url: md.url.clone(),
        username: name,
        password: dr.password(),
        form_action_origin: md.url,
        guid: Uuid::new_v4().to_string(),
        time_created: time::string_to_epoch(md.created),
        time_last_used: time::string_to_epoch(md.last_used),
        time_password_changed: time::string_to_epoch(md.password_changed),

        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let record = new("https://example.com".to_string(), "testuser".to_string());
        assert_eq!(record.url, "https://example.com");
        assert_eq!(record.username, "testuser");
        assert_eq!(record.password, "");
    }

    #[test]
    fn test_new_with_password() {
        let record = new_with_password(
            "https://example.com".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );
        assert_eq!(record.url, "https://example.com");
        assert_eq!(record.username, "testuser");
        assert_eq!(record.password, "testpass");
    }

    #[test]
    fn test_to_decrypted() {
        let record = new_with_password(
            "https://example.com".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );
        let decrypted = record.to_decrypted();
        assert_eq!(decrypted.secrets.user, "testuser");
        assert_eq!(decrypted.secrets.password, "testpass");
        assert_eq!(decrypted.metadata.url, "https://example.com");
        assert_eq!(decrypted.metadata.name, "testuser");
    }

    #[test]
    fn test_to_decrypted_with_timestamps() {
        let record = Record {
            url: "https://example.com".to_string(),
            username: "testuser".to_string(),
            password: "testpass".to_string(),
            time_created: 1609459200,
            time_last_used: 1609545600,
            time_password_changed: 1609632000,
            ..Default::default()
        };
        let decrypted = record.to_decrypted();
        assert!(!decrypted.metadata.created.is_empty());
        assert!(!decrypted.metadata.last_used.is_empty());
        assert!(!decrypted.metadata.password_changed.is_empty());
    }

    #[test]
    fn test_from_decrypted() {
        let mut decrypted = records::DecryptedRecord {
            secrets: secrets_from_user_pass("testuser", "testpass"),
            metadata: records::default_metadata(),
            history: Vec::new(),
        };
        decrypted.metadata.url = "https://example.com".to_string();
        decrypted.metadata.name = "testuser".to_string();

        let firefox_record = from_decrypted(decrypted.clone());
        assert_eq!(firefox_record.url, "https://example.com");
        assert_eq!(firefox_record.username, "testuser");
        assert_eq!(firefox_record.password, "testpass");
        assert!(!firefox_record.guid.is_empty());
    }

    #[test]
    fn test_from_decrypted_empty_name() {
        let mut decrypted = records::DecryptedRecord {
            secrets: secrets_from_user_pass("testuser", "testpass"),
            metadata: records::default_metadata(),
            history: Vec::new(),
        };
        decrypted.metadata.url = "https://example.com".to_string();
        decrypted.metadata.name = "".to_string();

        let firefox_record = from_decrypted(decrypted);
        assert_eq!(firefox_record.username, "testuser");
    }

    #[test]
    fn test_default_record() {
        let record = Record::default();
        assert_eq!(record.url, "");
        assert_eq!(record.username, "");
        assert_eq!(record.password, "");
        assert_eq!(record.time_created, 0);
    }
}


// use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Kind {
    #[default]
    Password
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EncryptedRecord {
    pub key: String,
    pub kind: Kind,
    pub value: Vec<u8>,
    pub created: String,
    pub updated: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PasswordValue {
    pub user: String,
    pub password: String,

}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecryptedRecord {
    pub key: String,
    pub kind: Kind,
    pub value: PasswordValue,
    pub created: String,
    pub updated: String,
}

impl DecryptedRecord {
    pub fn encrypt(&self) -> EncryptedRecord {
        EncryptedRecord{
            key: self.key.clone(),
            kind: self.kind.clone(),
            value: bincode::serialize(&self.value).unwrap(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }
}

impl EncryptedRecord {
    pub fn decrypt(&self) -> DecryptedRecord {
        DecryptedRecord{
            key: self.key.clone(),
            kind: self.kind.clone(),
            value: bincode::deserialize(&self.value[..]).unwrap(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
}
}

#[cfg(test)]
mod tests {
    use crate::store::record::{DecryptedRecord, Kind, PasswordValue};

    #[test]
    fn password_records() {
        let now = chrono::offset::Local::now().to_rfc3339();
        let dpr = DecryptedRecord{
            key: "a site".to_string(),
            kind: Kind::Password,
            value: PasswordValue{
                user: "alice@site.com".to_string(),
                password: "4 s3kr1t".to_string(),
            },
            created: now.clone(),
            updated: now.clone(),
        };
        let epr = dpr.encrypt();
        assert_eq!(
            epr.value,
            [14, 0, 0, 0, 0, 0, 0, 0, 97, 108, 105, 99, 101, 64, 115, 105, 116, 101, 46, 99, 111, 109, 8, 0, 0, 0, 0, 0, 0, 0, 52, 32, 115, 51, 107, 114, 49, 116]
        );
        let re_dpr = epr.decrypt();
        assert_eq!(re_dpr.value.password, "4 s3kr1t");
    }
}

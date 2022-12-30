use bincode::{config, Decode, Encode};
use secrecy::Zeroize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Kind {
    #[default]
    Password,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct EncryptedRecord {
    pub key: String,
    pub kind: Kind,
    pub value: Vec<u8>,
    pub created: String,
    pub updated: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct PasswordValue {
    pub user: String,
    pub password: String,
}

impl Zeroize for PasswordValue {
    fn zeroize(&mut self) {
        self.password.zeroize();
    }
}

impl std::fmt::Display for PasswordValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PasswordValue{{user: {}, password: *****}}", self.user)
    }
}

impl std::fmt::Debug for PasswordValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PasswordValue{{user: {}, password: *****}}", self.user)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub key: String,
    pub kind: Kind,
    pub value: PasswordValue,
    pub created: String,
    pub updated: String,
}

impl DecryptedRecord {
    pub fn encrypt(&self) -> EncryptedRecord {
        EncryptedRecord {
            key: self.key.clone(),
            kind: self.kind.clone(),
            value: bincode::encode_to_vec(&self.value, config::standard()).unwrap(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }
}

impl EncryptedRecord {
    pub fn decrypt(&self) -> DecryptedRecord {
        let (decoded, _len) =
            bincode::decode_from_slice(&self.value[..], config::standard()).unwrap();
        DecryptedRecord {
            key: self.key.clone(),
            kind: self.kind.clone(),
            value: decoded,
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
        let dpr = DecryptedRecord {
            key: "a site".to_string(),
            kind: Kind::Password,
            value: PasswordValue {
                user: "alice@site.com".to_string(),
                password: "4 s3kr1t".to_string(),
            },
            created: now.clone(),
            updated: now,
        };
        assert_eq!(
            format!("{}", dpr.value),
            "PasswordValue{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.value),
            "PasswordValue{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt();
        assert_eq!(
            epr.value,
            [
                14, 97, 108, 105, 99, 101, 64, 115, 105, 116, 101, 46, 99, 111, 109, 8, 52, 32,
                115, 51, 107, 114, 49, 116
            ]
        );
        let re_dpr = epr.decrypt();
        assert_eq!(re_dpr.value.password, "4 s3kr1t");
    }
}

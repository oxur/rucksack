use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::v050;
pub use super::v050::{Creds, DecryptedRecord, EncryptedRecord, Metadata};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Kind {
    #[default]
    Account,
    Credential,
    Password,
}

pub const DEFAULT_KIND: Kind = Kind::Password;

pub fn migrate_kind_from_v050(k: v050::Kind) -> Kind {
    match k {
        v050::Kind::Password => Kind::Password,
    }
}

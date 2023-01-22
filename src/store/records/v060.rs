use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::v020;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Kind {
    #[default]
    Account,
    Credential,
    Password,
}

pub const DEFAULT_KIND: Kind = Kind::Password;

pub fn migrate_kind_from_v020(k: v020::Kind) -> Kind {
    match k {
        v020::Kind::Password => Kind::Password,
        _ => Kind::Account,
    }
}

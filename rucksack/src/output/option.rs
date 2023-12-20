use rucksack_db::records::Kind;

use super::Column;

#[derive(Clone, Debug, Default)]
pub struct Opts {
    pub all_tags: Option<Vec<String>>,
    pub any_tags: Option<Vec<String>>,
    pub backup_files: bool,
    pub category: String,
    pub categories: bool,
    pub decrypted: bool,
    pub group_by_category: bool,
    pub group_by_kind: bool,
    pub group_by_name: bool,
    pub group_by_password: bool,
    pub group_by_hash: bool,
    pub hash_fields: Vec<Column>,
    pub kind: Kind,
    pub kinds: bool,
    pub latest_only: bool,
    pub only_deleted: bool,
    pub only_keys: bool,
    pub password_history: bool,
    pub reveal: bool,
    pub skip_deleted: bool,
    pub tags: bool,
    pub with_status: bool,
}

pub fn defaults() -> Opts {
    Opts {
        ..Default::default()
    }
}

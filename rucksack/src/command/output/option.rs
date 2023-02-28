#[derive(Clone, Debug, Default)]
pub struct Opts {
    pub backup_files: bool,
    pub categories: bool,
    pub decrypted: bool,
    pub group_by_category: bool,
    pub group_by_kind: bool,
    pub group_by_name: bool,
    pub group_by_password: bool,
    pub kinds: bool,
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

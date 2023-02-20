#[derive(Clone, Debug, Default)]
pub struct Opts {
    pub decrypted: bool,
    pub only_deleted: bool,
    pub reveal: bool,
    pub skip_deleted: bool,
    pub with_status: bool,
    pub password_history: bool,
    pub only_keys: bool,
    pub kinds: bool,
    pub categories: bool,
    pub tags: bool,
}

pub fn defaults() -> Opts {
    Opts {
        ..Default::default()
    }
}

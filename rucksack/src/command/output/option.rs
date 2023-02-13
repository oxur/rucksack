#[derive(Clone, Debug, Default)]
pub struct Opts {
    pub decrypted: bool,
    pub only_deleted: bool,
    pub reveal: bool,
    pub skip_deleted: bool,
    pub with_status: bool,
}

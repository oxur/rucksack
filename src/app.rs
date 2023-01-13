use crate::{config, store};

#[derive(Debug)]
pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

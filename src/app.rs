use crate::{config, store};

pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

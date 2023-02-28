pub mod backup;
pub mod encrypted;
pub mod store;
pub mod versioned;

pub use store::{init, new, open, DB};

use crate::records;

pub fn version() -> versions::SemVer {
    records::version()
}

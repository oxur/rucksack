pub mod backup;
pub mod encrypted;
pub mod manager;
pub mod versioned;

pub use manager::DB;

use crate::records;

pub fn version() -> versions::SemVer {
    records::version()
}

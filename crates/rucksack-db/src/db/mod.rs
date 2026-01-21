pub mod encrypted;
pub mod manager;
pub mod versioned;

pub use manager::DB;

use crate::records;

pub fn version() -> versions::SemVer {
    records::version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version.major, 0);
        assert!(version.minor >= 9);
    }
}

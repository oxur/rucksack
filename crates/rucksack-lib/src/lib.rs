pub mod file;
pub mod generator;
pub mod time;
pub mod util;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION"))
        .expect("CARGO_PKG_VERSION must be valid semver format")
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

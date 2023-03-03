pub mod file;
pub mod generator;
pub mod time;
pub mod util;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}

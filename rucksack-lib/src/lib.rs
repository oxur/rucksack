pub mod config;
pub mod file;
pub mod generator;
pub mod time;
pub mod util;

pub use config::Config;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}

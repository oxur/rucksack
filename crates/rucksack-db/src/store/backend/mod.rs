pub mod backup;
#[cfg(feature = "filesystem")]
pub mod filesystem;
#[cfg(feature = "redb")]
pub mod redb;

#[cfg(feature = "filesystem")]
pub use crate::store::backend::filesystem::FileSystemBackend;
// When both features are enabled, filesystem takes precedence, making redb unused
#[cfg(feature = "redb")]
#[cfg_attr(all(feature = "redb", feature = "filesystem"), allow(unused_imports))]
pub use crate::store::backend::redb::ReDBBackend;

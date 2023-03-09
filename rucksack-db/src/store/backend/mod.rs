pub mod backup;
#[cfg(feature = "filesystem")]
pub mod filesystem;
#[cfg(feature = "persy")]
pub mod persy;
#[cfg(feature = "redb")]
pub mod redb;

#[cfg(feature = "filesystem")]
pub use crate::store::backend::filesystem::FileSystemBackend;
#[cfg(feature = "persy")]
pub use crate::store::backend::persy::PersyBackend;
#[cfg(feature = "redb")]
pub use crate::store::backend::redb::ReDBBackend;

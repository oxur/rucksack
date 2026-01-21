//! # (a support module)
pub mod column;
pub mod option;
pub mod result;
pub mod table;

pub use column::Column;
pub use option::Opts;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_reexport() {
        let col = Column::Name;
        assert_eq!(format!("{col}"), "Name");
    }

    #[test]
    fn test_opts_reexport() {
        let opts = Opts::default();
        assert!(!opts.backup_files);
    }
}

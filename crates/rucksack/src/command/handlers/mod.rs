pub mod add;
pub mod backup;
pub mod completions;
pub mod config;
pub mod dedupe;
pub mod delete;
pub mod export;
pub mod gen;
#[doc(hidden)]
pub mod help;
pub mod import;
pub mod list;
pub mod set;
pub mod show;
#[doc(hidden)]
pub mod version;

pub use completions::completions;
pub use help::long_help;
pub use version::version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexport_version() {
        // Test that version re-export works
        let result = version();
        assert!(result.is_ok());
    }

    #[test]
    fn test_reexport_long_help() {
        use clap::Command;
        let cmd = Command::new("test");
        let result = long_help(cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reexport_completions() {
        use clap::Command;
        use clap_complete::Shell;
        let cmd = Command::new("test");
        let result = completions(Shell::Bash, cmd, "test".to_string());
        assert!(result.is_ok());
    }
}

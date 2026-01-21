use anyhow::{Context, Result};
use clap::Command;

pub fn long_help(mut cmd: Command) -> Result<()> {
    cmd.print_long_help()
        .with_context(|| "failed to print help".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_help() {
        let cmd = Command::new("test")
            .about("Test command")
            .arg(clap::Arg::new("arg1").help("First argument"));
        let result = long_help(cmd);
        assert!(result.is_ok());
    }
}

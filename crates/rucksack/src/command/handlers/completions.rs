//! # Shell Completion
//!
//! To be able to tab-complete the rucksack CLI, do the following:
//!
//! ```shell
//! rucksack completions --bash > ~/.rucksack_bash_completion
//!
//! echo ". ~/.rucksack_bash_completion" >> ~/.profile
//! ```
//!
//! Note that, while `bash` is used here as an example, the following shells
//! are supported:
//!
//! * bash
//! * elvish
//! * fish
//! * powershell
//! * zsh
//!
use std::io;

use anyhow::Result;
use clap::Command;
use clap_complete::Shell;

pub fn completions(shell: Shell, mut cmd: Command, name: String) -> Result<()> {
    clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completions_bash() {
        let cmd = Command::new("test");
        let result = completions(Shell::Bash, cmd, "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_completions_zsh() {
        let cmd = Command::new("test");
        let result = completions(Shell::Zsh, cmd, "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_completions_fish() {
        let cmd = Command::new("test");
        let result = completions(Shell::Fish, cmd, "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_completions_powershell() {
        let cmd = Command::new("test");
        let result = completions(Shell::PowerShell, cmd, "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_completions_elvish() {
        let cmd = Command::new("test");
        let result = completions(Shell::Elvish, cmd, "test".to_string());
        assert!(result.is_ok());
    }
}

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
use std::io::Stdout;

use anyhow::Result;
use clap::Command;
use clap_complete::Shell;

pub fn completions(
    shell: Shell,
    mut cmd: Command,
    name: String,
    mut output: &Stdout,
) -> Result<()> {
    clap_complete::generate(shell, &mut cmd, name, &mut output);
    Ok(())
}

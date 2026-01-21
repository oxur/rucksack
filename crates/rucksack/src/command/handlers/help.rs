use anyhow::{Context, Result};
use clap::Command;

pub fn long_help(mut cmd: Command) -> Result<()> {
    cmd.print_long_help()
        .with_context(|| "failed to print help".to_string())
}

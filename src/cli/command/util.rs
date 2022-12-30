use anyhow::{Result};

pub fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

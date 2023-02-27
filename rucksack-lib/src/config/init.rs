use anyhow::Result;

use crate::file;

const DEFAULT_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = true
"#;

pub fn config(filename: String) -> Result<()> {
    let file_path = file::create_parents(filename.clone())?;
    if file_path.exists() {
        return Ok(());
    }
    file::write(DEFAULT_TOML.as_bytes().to_vec(), filename)
}

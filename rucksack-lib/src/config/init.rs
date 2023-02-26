use anyhow::Result;

use crate::util;

const DEFAULT_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = true
"#;

pub fn config(filename: String) -> Result<()> {
    let file_path = util::create_parents(filename.clone())?;
    if file_path.exists() {
        return Ok(());
    }
    util::write_file(DEFAULT_TOML.as_bytes().to_vec(), filename)
}

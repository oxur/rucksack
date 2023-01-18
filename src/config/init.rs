use std::{fs, path};

use anyhow::Result;

use crate::util;

const DEFAULT_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "debug"
report_caller = true
"#;

pub fn config(filename: String) -> Result<()> {
    let file_path = path::Path::new(&filename);
    if file_path.exists() {
        return Ok(());
    }
    fs::create_dir_all(file_path.parent().unwrap())?;
    util::write_file(DEFAULT_TOML.as_bytes().to_vec(), filename)
}

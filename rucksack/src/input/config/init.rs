use anyhow::Result;

use rucksack_lib::file;

const DEFAULT_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = true

[retention]
purge_on_shutdown = false
archive_deletes = true
delete_inactive = false

[output]
show_inactive = true
show_deleted = false
"#;

#[derive(Clone, Default)]
struct Opts {
    force: bool,
}

fn default_opts() -> Opts {
    Opts {
        ..Default::default()
    }
}

pub fn config(filename: String) -> Result<()> {
    initialise(filename, default_opts())
}

pub fn recreate(filename: String) -> Result<()> {
    initialise(filename, Opts { force: true })
}

fn initialise(filename: String, opts: Opts) -> Result<()> {
    let file_path = file::create_parents(filename.clone())?;
    if file_path.exists() && !opts.force {
        log::debug!("File already exists; skipping init ...");
        return Ok(());
    }
    file::write(DEFAULT_TOML.as_bytes().to_vec(), filename)
}

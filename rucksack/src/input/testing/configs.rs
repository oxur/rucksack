pub const PURGE_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = false

[retention]
purge_on_shutdown = true
archive_deletes = true
delete_inactive = false

[output]
show_inactive = true
show_deleted = false
"#;

pub const DELETE_INACTIVE_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = false

[retention]
purge_on_shutdown = false
archive_deletes = true
delete_inactive = true

[output]
show_inactive = true
show_deleted = false
"#;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purge_toml() {
        assert!(PURGE_TOML.contains("purge_on_shutdown = true"));
        assert!(PURGE_TOML.contains("[retention]"));
    }

    #[test]
    fn test_delete_inactive_toml() {
        assert!(DELETE_INACTIVE_TOML.contains("delete_inactive = true"));
        assert!(DELETE_INACTIVE_TOML.contains("[retention]"));
    }
}

use clap::Arg;

pub fn path() -> Arg {
    Arg::new("db")
        .help("Path to the encrypted database to use")
        .short('d')
        .long("db")
        .env("RUXAK_DB")
        .global(true)
}

pub fn pwd() -> Arg {
    Arg::new("db-pass")
        .help("Password used to encrypt the database")
        .long("db-pass")
        .env("RUXAK_DB_PASS")
        .global(true)
}

pub fn salt() -> Arg {
    Arg::new("salt")
        .help("The salt to use for encrypting the database")
        .long("salt")
        .env("RUXAK_SALT")
        .global(true)
}

pub fn backup_dir() -> Arg {
    Arg::new("backup-dir")
        .help("Path for database backups")
        .long("backup-dir")
        .env("RUXAK_BACKUP_DIR")
        .global(true)
}

// TODO: let's look at the other bool flags and make sure we're being consistent
pub fn not_needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("false")
        .global(true)
}

// TODO: let's look at the other bool flags and make sure we're being consistent
pub fn needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("true")
        .global(true)
}

pub fn serialised_format() -> Arg {
    Arg::new("format")
        .help("the de/serialisation format to use for import/export")
        .long("format")
        .env("RUXAK_FORMAT")
        .value_parser(["", "chrome", "debug", "firefox"])
        .global(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let arg = path();
        assert_eq!(arg.get_id(), "db");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_pwd() {
        let arg = pwd();
        assert_eq!(arg.get_id(), "db-pass");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_salt() {
        let arg = salt();
        assert_eq!(arg.get_id(), "salt");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_backup_dir() {
        let arg = backup_dir();
        assert_eq!(arg.get_id(), "backup-dir");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_not_needed() {
        let arg = not_needed();
        assert_eq!(arg.get_id(), "db-needed");
        assert!(arg.is_global_set());
        assert!(arg.is_hide_set());
    }

    #[test]
    fn test_needed() {
        let arg = needed();
        assert_eq!(arg.get_id(), "db-needed");
        assert!(arg.is_global_set());
        assert!(arg.is_hide_set());
    }

    #[test]
    fn test_serialised_format() {
        let arg = serialised_format();
        assert_eq!(arg.get_id(), "format");
        assert!(arg.is_global_set());
    }
}

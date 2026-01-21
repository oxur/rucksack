pub const DEFAULT_LOG_LEVEL: &str = "error";
pub const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const SALT_ENV: &str = "USER";
pub const SALT_FALLBACK: &str = "rucksack";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_log_level() {
        assert_eq!(DEFAULT_LOG_LEVEL, "error");
    }

    #[test]
    fn test_desc() {
        assert!(!DESC.is_empty());
    }

    #[test]
    fn test_name() {
        assert_eq!(NAME, "rucksack");
    }

    #[test]
    fn test_salt_env() {
        assert_eq!(SALT_ENV, "USER");
    }

    #[test]
    fn test_salt_fallback() {
        assert_eq!(SALT_FALLBACK, "rucksack");
    }
}

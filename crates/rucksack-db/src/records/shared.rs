// This function is here for backwards compatibility, used only by versions
// prior to 0.7.0. For versions 0.7.0 and latter, see the version-specific
// records/*.rs file.
pub fn key(user: &str, url: &str) -> String {
    format!("{user}:{url}")
}

pub fn version(v: &str) -> versions::SemVer {
    trim_version(versions::SemVer::new(v).unwrap())
}

pub fn trim_version(sv: versions::SemVer) -> versions::SemVer {
    // dev versions and release candidates throw off version comparisons, so we drop those:
    versions::SemVer {
        major: sv.major,
        minor: sv.minor,
        patch: sv.patch,
        pre_rel: None,
        meta: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_basic() {
        assert_eq!(key("user1", "example.com"), "user1:example.com");
    }

    #[test]
    fn test_key_empty_user() {
        assert_eq!(key("", "example.com"), ":example.com");
    }

    #[test]
    fn test_key_empty_url() {
        assert_eq!(key("user1", ""), "user1:");
    }

    #[test]
    fn test_key_both_empty() {
        assert_eq!(key("", ""), ":");
    }

    #[test]
    fn test_key_special_characters() {
        assert_eq!(
            key("user@email.com", "https://example.com/path"),
            "user@email.com:https://example.com/path"
        );
    }

    #[test]
    fn test_key_unicode() {
        assert_eq!(key("用户", "例え.com"), "用户:例え.com");
    }

    #[test]
    fn test_version_basic() {
        let v = version("1.2.3");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert!(v.pre_rel.is_none());
        assert!(v.meta.is_none());
    }

    #[test]
    fn test_version_with_prerelease() {
        let v = version("2.0.0-beta.1");
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
        assert!(v.pre_rel.is_none(), "Pre-release should be stripped");
        assert!(v.meta.is_none());
    }

    #[test]
    fn test_version_with_metadata() {
        let v = version("1.0.0+20130313144700");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
        assert!(v.pre_rel.is_none());
        assert!(v.meta.is_none(), "Metadata should be stripped");
    }

    #[test]
    fn test_version_zero() {
        let v = version("0.0.0");
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn test_version_large_numbers() {
        let v = version("999.888.777");
        assert_eq!(v.major, 999);
        assert_eq!(v.minor, 888);
        assert_eq!(v.patch, 777);
    }

    #[test]
    fn test_trim_version_basic() {
        let original = versions::SemVer::new("1.2.3").unwrap();
        let trimmed = trim_version(original.clone());
        assert_eq!(trimmed.major, 1);
        assert_eq!(trimmed.minor, 2);
        assert_eq!(trimmed.patch, 3);
        assert!(trimmed.pre_rel.is_none());
        assert!(trimmed.meta.is_none());
    }

    #[test]
    fn test_trim_version_removes_prerelease() {
        let original = versions::SemVer::new("1.0.0-alpha").unwrap();
        let trimmed = trim_version(original);
        assert_eq!(trimmed.major, 1);
        assert_eq!(trimmed.minor, 0);
        assert_eq!(trimmed.patch, 0);
        assert!(
            trimmed.pre_rel.is_none(),
            "Pre-release should be removed by trim"
        );
    }

    #[test]
    fn test_trim_version_removes_metadata() {
        let original = versions::SemVer::new("2.1.0+build.123").unwrap();
        let trimmed = trim_version(original);
        assert_eq!(trimmed.major, 2);
        assert_eq!(trimmed.minor, 1);
        assert_eq!(trimmed.patch, 0);
        assert!(trimmed.meta.is_none(), "Metadata should be removed by trim");
    }

    #[test]
    fn test_trim_version_removes_both() {
        let original = versions::SemVer::new("3.2.1-rc.1+build").unwrap();
        let trimmed = trim_version(original);
        assert_eq!(trimmed.major, 3);
        assert_eq!(trimmed.minor, 2);
        assert_eq!(trimmed.patch, 1);
        assert!(trimmed.pre_rel.is_none());
        assert!(trimmed.meta.is_none());
    }

    #[test]
    fn version_comparisons() {
        assert!(version("1.1.0") > version("1.0.0"));
        assert_eq!(version("1.1.0"), version("1.1.0-RC1"));
        assert_eq!(version("1.1.0"), version("1.1.0-dev"));
        assert!(version("1.1.0-dev") > version("1.0.9"));
    }

    #[test]
    fn test_version_comparisons_extended() {
        assert!(version("2.0.0") > version("1.9.9"));
        assert!(version("1.0.1") > version("1.0.0"));
        assert!(version("1.1.0") > version("1.0.9"));
        assert_eq!(version("1.0.0"), version("1.0.0"));
        assert_eq!(version("1.0.0-alpha"), version("1.0.0-beta"));
        assert_eq!(version("1.0.0+build1"), version("1.0.0+build2"));
    }

    #[test]
    fn test_version_with_dev_suffix() {
        let v1 = version("0.9.0-dev");
        let v2 = version("0.9.0");
        assert_eq!(v1, v2, "Dev suffix should be ignored in comparison");
    }

    #[test]
    fn test_version_with_rc_suffix() {
        let v1 = version("1.0.0-RC1");
        let v2 = version("1.0.0-RC2");
        let v3 = version("1.0.0");
        assert_eq!(v1, v2, "RC versions should compare equal when trimmed");
        assert_eq!(v1, v3, "RC version should equal release version");
    }
}

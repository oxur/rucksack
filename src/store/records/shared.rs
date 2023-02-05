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

    #[test]
    fn version_comparisons() {
        assert!(super::version("1.1.0") > super::version("1.0.0"));
        assert_eq!(super::version("1.1.0"), super::version("1.1.0-RC1"));
        assert_eq!(super::version("1.1.0"), super::version("1.1.0-dev"));
        assert!(super::version("1.1.0-dev") > super::version("1.0.9"));
    }
}

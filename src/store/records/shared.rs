pub fn key(user: &str, url: &str) -> String {
    format!("{}:{}", user, url)
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
    use crate::store::records::shared::version;

    #[test]
    fn version_comparisons() {
        assert!(version("1.1.0") > version("1.0.0"));
        assert_eq!(version("1.1.0"), version("1.1.0-RC1"));
        assert_eq!(version("1.1.0"), version("1.1.0-dev"));
        assert!(version("1.1.0-dev") > version("1.0.9"));
    }
}

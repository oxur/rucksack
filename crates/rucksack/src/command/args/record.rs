use clap::{value_parser, Arg, ArgAction};

pub fn category() -> Arg {
    Arg::new("category")
        .help("The user-supplied category of the given record")
        .long("category")
        .env("RUXAK_CATEGORY")
        .global(true)
}

pub fn status() -> Arg {
    Arg::new("status")
        .help("The status of the given record")
        .env("RUXAK_STATUS")
        .value_parser(["active", "inactive", "deleted"])
}

pub fn types_allowed() -> Vec<&'static str> {
    vec![
        "",
        "account",
        "asymmetric-crypto",
        "asymmetric",
        "certs",
        "certificates",
        "password",
        "service-creds",
        "service-credentials",
    ]
}

pub fn types_list_allowed() -> Vec<&'static str> {
    let mut rta = types_allowed();
    rta.push("any");
    rta
}

pub fn base_type() -> Arg {
    Arg::new("type")
        .help("The type of secret for the record")
        .short('t')
        .long("type")
        .env("RUXAK_TYPE")
        .global(true)
}

pub fn kind() -> Arg {
    base_type().value_parser(types_allowed())
}

pub fn type_list() -> Arg {
    // Note that the default value in this arg is a way of working around
    // using a common base type for several args, and does not have anything
    // to do with configuration
    base_type()
        .default_value("any")
        .value_parser(types_list_allowed())
}

pub fn name() -> Arg {
    Arg::new("name")
        .help("the record name")
        .long("name")
        .env("RUXAK_NAME")
}

pub fn user() -> Arg {
    Arg::new("user")
        .help("The user/login identifier")
        .short('u')
        .long("user")
        .env("RUXAK_USER")
}

pub fn user_old() -> Arg {
    Arg::new("old-user")
        .help("The old user login name")
        .long("old-user")
}

pub fn user_new() -> Arg {
    Arg::new("new-user")
        .help("The new user login name to use")
        .long("new-user")
}

pub fn pass() -> Arg {
    Arg::new("password")
        .help("The login password")
        .long("password")
        .env("RUXAK_PASSWORD")
}

pub fn url() -> Arg {
    Arg::new("url")
        .help("the login URL")
        .long("url")
        .env("RUXAK_URL")
}

pub fn url_old() -> Arg {
    Arg::new("old-url")
        .help("The old login URL")
        .long("old-url")
}

pub fn url_new() -> Arg {
    Arg::new("new-url")
        .help("The new URL for the login")
        .long("new-url")
}

pub fn account_id() -> Arg {
    Arg::new("account-id")
        .help("The account ID for secrets of type 'account'")
        .long("account-id")
        .env("RUXAK_ACCOUNT_ID")
}

pub fn secret_public() -> Arg {
    Arg::new("public")
        .help("The public key for asymmetric-crypto secrets; the public cert for certificate-based secrets")
        .long("public")
        .env("RUXAK_PUBLIC_KEY")
}

pub fn secret_private() -> Arg {
    Arg::new("private")
        .help("The private key for asymmetric-crypto secrets; the private cert for certificate-based secrets")
        .long("private")
        .env("RUXAK_PRIVATE_KEY")
}

pub fn root_cert() -> Arg {
    Arg::new("root")
        .help("The root cert for certificate-based secrets")
        .long("root")
        .env("RUXAK_ROOT_CERT")
}

pub fn key() -> Arg {
    Arg::new("key")
        .help("The key for service-credential-based secrets")
        .long("key")
        .env("RUXAK_KEY")
}

pub fn secret() -> Arg {
    Arg::new("secret")
        .help("The secret for service-credential-based secrets")
        .long("secret")
        .env("RUXAK_SECRET")
}

pub fn tags() -> Arg {
    Arg::new("tags")
        .help("One or more tags for a record (use a ',' to delimit multiple)")
        .long("tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

pub fn all_tags() -> Arg {
    Arg::new("all-tags")
        .help("Limit results to records that have ALL of the tags passed")
        .long("all-tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

pub fn any_tags() -> Arg {
    Arg::new("any-tags")
        .help("Limit results to records that have ANY of the tags passed")
        .long("any-tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_allowed() {
        assert_eq!(
            super::types_allowed(),
            vec![
                "",
                "account",
                "asymmetric-crypto",
                "asymmetric",
                "certs",
                "certificates",
                "password",
                "service-creds",
                "service-credentials",
            ]
        );
    }

    #[test]
    fn types_list_allowed() {
        assert_eq!(
            super::types_list_allowed(),
            vec![
                "",
                "account",
                "asymmetric-crypto",
                "asymmetric",
                "certs",
                "certificates",
                "password",
                "service-creds",
                "service-credentials",
                "any"
            ]
        );
    }

    #[test]
    fn test_category() {
        let arg = category();
        assert_eq!(arg.get_id(), "category");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_status() {
        let arg = status();
        assert_eq!(arg.get_id(), "status");
    }

    #[test]
    fn test_base_type() {
        let arg = base_type();
        assert_eq!(arg.get_id(), "type");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_kind() {
        let arg = kind();
        assert_eq!(arg.get_id(), "type");
    }

    #[test]
    fn test_type_list() {
        let arg = type_list();
        assert_eq!(arg.get_id(), "type");
    }

    #[test]
    fn test_name() {
        let arg = name();
        assert_eq!(arg.get_id(), "name");
    }

    #[test]
    fn test_user() {
        let arg = user();
        assert_eq!(arg.get_id(), "user");
        // Note: user() is not global to avoid clap's "Global arguments cannot be required" error
    }

    #[test]
    fn test_user_old() {
        let arg = user_old();
        assert_eq!(arg.get_id(), "old-user");
    }

    #[test]
    fn test_user_new() {
        let arg = user_new();
        assert_eq!(arg.get_id(), "new-user");
    }

    #[test]
    fn test_pass() {
        let arg = pass();
        assert_eq!(arg.get_id(), "password");
    }

    #[test]
    fn test_url() {
        let arg = url();
        assert_eq!(arg.get_id(), "url");
        // Note: url() is not global to avoid clap's "Global arguments cannot be required" error
    }

    #[test]
    fn test_url_old() {
        let arg = url_old();
        assert_eq!(arg.get_id(), "old-url");
    }

    #[test]
    fn test_url_new() {
        let arg = url_new();
        assert_eq!(arg.get_id(), "new-url");
    }

    #[test]
    fn test_account_id() {
        let arg = account_id();
        assert_eq!(arg.get_id(), "account-id");
    }

    #[test]
    fn test_secret_public() {
        let arg = secret_public();
        assert_eq!(arg.get_id(), "public");
    }

    #[test]
    fn test_secret_private() {
        let arg = secret_private();
        assert_eq!(arg.get_id(), "private");
    }

    #[test]
    fn test_root_cert() {
        let arg = root_cert();
        assert_eq!(arg.get_id(), "root");
    }

    #[test]
    fn test_key() {
        let arg = key();
        assert_eq!(arg.get_id(), "key");
    }

    #[test]
    fn test_secret() {
        let arg = secret();
        assert_eq!(arg.get_id(), "secret");
    }

    #[test]
    fn test_tags() {
        let arg = tags();
        assert_eq!(arg.get_id(), "tags");
    }

    #[test]
    fn test_all_tags() {
        let arg = all_tags();
        assert_eq!(arg.get_id(), "all-tags");
    }

    #[test]
    fn test_any_tags() {
        let arg = any_tags();
        assert_eq!(arg.get_id(), "any-tags");
    }
}

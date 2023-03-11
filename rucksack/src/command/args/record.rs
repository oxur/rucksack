use clap::{value_parser, Arg, ArgAction};

use rucksack_db::records;

pub fn category() -> Arg {
    Arg::new("category")
        .help("The user-supplied category of the given record")
        .long("category")
        .default_value(records::DEFAULT_CATEGORY)
        .env("RUXAK_CATEGORY")
        .global(true)
}

pub fn status() -> Arg {
    Arg::new("status")
        .help("The status of the given record")
        .default_value("active")
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
    base_type()
        .default_value("password")
        .value_parser(types_allowed())
}

pub fn type_list() -> Arg {
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
        .short('u')
        .long("old-user")
}

pub fn user_new() -> Arg {
    Arg::new("new-user")
        .help("The new user login name to use")
        .short('u')
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

mod tests {

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
}

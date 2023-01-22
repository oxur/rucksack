pub fn key(user: &str, url: &str) -> String {
    format!("{}:{}", user, url)
}

use serde::Deserialize;

use super::firefox;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Record {
    pub fn to_firefox(&self) -> firefox::Record {
        firefox::new_with_password(
            self.url.clone(),
            self.username.clone(),
            self.password.clone(),
        )
    }
}

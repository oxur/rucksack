use anyhow::{anyhow, Result};
use secrecy::SecretString;

pub fn secret(prompt: &str) -> Result<SecretString> {
    rpassword::prompt_password(prompt)
        .map(SecretString::new)
        .map_err(|e| anyhow!("password prompt failed: {}", e.to_string()))
}

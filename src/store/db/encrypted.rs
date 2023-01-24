use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, Secret, SecretString};

use crate::store::crypto;
use crate::util;

pub struct EncryptedDB {
    bytes: Vec<u8>,
    decrypted: Secret<Vec<u8>>,
    path: String,
    pwd: SecretString,
    salt: SecretString,
}

pub fn from_bytes(decrypted: Vec<u8>, path: String, pwd: String, salt: String) -> EncryptedDB {
    new(Vec::new(), decrypted, path, pwd, salt)
}

pub fn from_file(path: String, pwd: String, salt: String) -> EncryptedDB {
    from_bytes(Vec::new(), path, pwd, salt)
}

pub fn new(
    bytes: Vec<u8>,
    decrypted: Vec<u8>,
    path: String,
    pwd: String,
    salt: String,
) -> EncryptedDB {
    EncryptedDB {
        bytes,
        decrypted: Secret::new(decrypted),
        path,
        pwd: SecretString::new(pwd),
        salt: SecretString::new(salt),
    }
}

impl EncryptedDB {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn decrypt(&mut self) -> Result<()> {
        match crypto::decrypt(self.bytes.clone(), self.pwd(), self.salt()) {
            Ok(bytes) => {
                log::trace!("decrypted bytes: {:?}", bytes);
                self.decrypted = Secret::new(bytes);
                Ok(())
            }
            Err(e) => {
                let msg = format!("could not decrypt data: {:?}", e);
                log::error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn decrypted(&self) -> Vec<u8> {
        self.decrypted.expose_secret().to_vec()
    }

    pub fn encrypt(&mut self) {
        self.bytes = crypto::encrypt(self.decrypted(), self.pwd(), self.salt());
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn pwd(&self) -> String {
        self.pwd.expose_secret().to_string()
    }

    pub fn read(&mut self) -> Result<()> {
        self.bytes = util::read_file(self.path())?;
        Ok(())
    }

    pub fn salt(&self) -> String {
        self.salt.expose_secret().to_string()
    }

    pub fn write(&self) -> Result<()> {
        util::write_file(self.bytes(), self.path())
    }
}

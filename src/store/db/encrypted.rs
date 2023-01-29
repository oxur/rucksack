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

pub fn from_decrypted(
    decrypted: Vec<u8>,
    path: String,
    pwd: String,
    salt: String,
) -> Result<EncryptedDB> {
    new(None, Some(decrypted), path, pwd, salt)
}

pub fn from_encrypted(
    encrypted: Vec<u8>,
    path: String,
    pwd: String,
    salt: String,
) -> Result<EncryptedDB> {
    new(Some(encrypted), None, path, pwd, salt)
}

pub fn from_file(path: String, pwd: String, salt: String) -> Result<EncryptedDB> {
    new(None, None, path, pwd, salt)
}

pub fn new(
    bytes: Option<Vec<u8>>,
    decrypted: Option<Vec<u8>>,
    path: String,
    pwd: String,
    salt: String,
) -> Result<EncryptedDB> {
    let mut edb = EncryptedDB {
        bytes: Vec::new(),
        decrypted: Secret::new(Vec::new()),
        path,
        pwd: SecretString::new(pwd),
        salt: SecretString::new(salt),
    };
    if bytes.is_none() && decrypted.is_none() {
        edb.read()?;
        edb.decrypt()?;
    } else if let Some(b) = bytes {
        edb.bytes = b;
        edb.decrypt()?;
    } else if let Some(d) = decrypted {
        edb.decrypted = Secret::new(d);
        edb.encrypt();
    }
    Ok(edb)
}

impl EncryptedDB {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn decrypt(&mut self) -> Result<()> {
        match crypto::decrypt(self.bytes.clone(), self.pwd(), self.salt()) {
            Ok(bytes) => {
                self.decrypted = Secret::new(bytes);
                Ok(())
            }
            Err(e) => {
                let msg = format!("Could not decrypt data: {:?}", e);
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

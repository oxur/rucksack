use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, Secret, SecretString};

use rucksack_lib::file;

use crate::crypto;

pub struct EncryptedDB {
    bytes: Vec<u8>,
    decrypted: Secret<Vec<u8>>,
    path: String,
    pwd: SecretString,
    salt: SecretString,
}

impl EncryptedDB {
    pub fn from_decrypted(
        decrypted: Vec<u8>,
        path: String,
        pwd: String,
        salt: String,
    ) -> Result<EncryptedDB> {
        EncryptedDB::new(None, Some(decrypted), path, pwd, salt)
    }

    pub fn from_encrypted(
        encrypted: Vec<u8>,
        path: String,
        pwd: String,
        salt: String,
    ) -> Result<EncryptedDB> {
        EncryptedDB::new(Some(encrypted), None, path, pwd, salt)
    }

    pub fn from_file(path: String, pwd: String, salt: String) -> Result<EncryptedDB> {
        EncryptedDB::new(None, None, path, pwd, salt)
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
            log::debug!("No bytes provided; reading from file ...");
            edb.read()?;
            edb.decrypt()?;
        } else if let Some(b) = bytes {
            log::debug!("Got encrypted bytes; decrypting ...");
            edb.bytes = b;
            edb.decrypt()?;
        } else if let Some(d) = decrypted {
            log::debug!("Got decrypted bytes; encrypting ...");
            edb.decrypted = Secret::new(d);
            edb.encrypt();
        }
        Ok(edb)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn decrypt(&mut self) -> Result<()> {
        log::debug!("Decrypting stored bytes ...");
        log::trace!("{}, {}", self.pwd(), self.salt());
        match crypto::decrypt(self.bytes.clone(), self.pwd(), self.salt()) {
            Ok(bytes) => {
                log::trace!("Decrypted bytes: {:?}", bytes);
                self.decrypted = Secret::new(bytes);
                Ok(())
            }
            Err(e) => {
                let msg = format!("Could not decrypt data: {e:?}");
                log::error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn decrypted(&self) -> Vec<u8> {
        self.decrypted.expose_secret().to_vec()
    }

    pub fn encrypt(&mut self) {
        log::trace!("Byte len before: {}", self.bytes.len());
        self.bytes = crypto::encrypt(self.decrypted(), self.pwd(), self.salt());
        log::trace!("Byte len after: {}", self.bytes.len());
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn pwd(&self) -> String {
        self.pwd.expose_secret().to_string()
    }

    pub fn read(&mut self) -> Result<()> {
        log::trace!("Byte len before: {}", self.bytes.len());
        self.bytes = file::read(self.path())?;
        log::trace!("Byte len after: {}", self.bytes.len());
        Ok(())
    }

    pub fn salt(&self) -> String {
        self.salt.expose_secret().to_string()
    }

    pub fn write(&self) -> Result<()> {
        log::debug!("Writing encrypted DB ...");
        file::write(self.bytes(), self.path())
    }
}

use age::secrecy::Secret;
use std::io::{Read, Write};
use age::{EncryptError, DecryptError};

#[derive(Clone)]
pub struct Encryption {
    secret: Secret<String>,
}

impl Encryption {
    pub fn new<S>(passphrase: S) -> Self where 
    S: ToString {
        let secret = Secret::new(passphrase.to_string());
        Encryption {
            secret,
        }
    }

    pub fn encrypt<S: AsRef<str>>(&self, input: S) -> Result<Vec<u8>, EncryptError> {
        let encryptor = age::Encryptor::with_user_passphrase(self.secret.clone());

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
        writer.write_all(input.as_ref().as_bytes())?;
        writer.finish()?;

        Ok(encrypted)
    }

    pub fn decrypt(&self, encrypted: Vec<u8>) -> Result<String, DecryptError> {
        let decryptor = match age::Decryptor::new(&encrypted as &[u8]) {
            Ok(age::Decryptor::Passphrase(d)) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(&self.secret, None)?;
        reader.read_to_end(&mut decrypted)?;

        Ok(String::from_utf8(decrypted).unwrap())
    }
}

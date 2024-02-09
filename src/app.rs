use std::fs;

use crate::Args;

mod encryption;
pub mod file;
use encryption::Encryption;
use file::Directory;

pub struct App {
    encryption: Encryption,
    directory: Directory,
}

#[derive(Debug)]
pub enum AppError {
    IncorrectPassword,
}

impl App {
    pub fn new(args: Args, passphrase: String) -> Self {
        App {
            encryption: Encryption::new(passphrase),
            directory: Directory::new(args.path).unwrap(),
        }
    }

    pub fn test_passphrase(&mut self) -> Result<(), AppError> {
        if let Some(first_path) = self.directory.nth_path(0) {
            let decrypted = match fs::read(&first_path) {
                Ok(encrypted) => self.encryption.decrypt(encrypted),
                Err(_) => return Ok(()),
            };
            if decrypted.is_err() {
                return Err(AppError::IncorrectPassword);
            }
        }
        Ok(())
        
    }
}

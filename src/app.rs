use std::{fs, path::PathBuf, ffi::OsString};

use crate::Args;

mod encryption;
pub mod file;
use encryption::Encryption;
use file::Directory;

pub struct App {
    encryption: Option<Encryption>,
    directory: Directory,
    journals_to_add: Vec<String>,
}

#[derive(Debug)]
pub enum AppError {
    IncorrectPassword,
    NoPassphrase,
}

pub fn encryption_from_option_passphrase(passphrase: Option<String>) -> Option<Encryption> {
    match passphrase {
        Some(passphrase) => Some(Encryption::new(passphrase)),
        None => None,
    }
}

impl App {
    pub fn new(args:Args) -> Self {
        let app = App {
            journals_to_add: args.add,
            encryption: encryption_from_option_passphrase(args.passphrase),
            directory: Directory::new(args.path).unwrap(),
        };
        app.add_journals();
        app
    }

    pub fn len(&self) -> usize {
        self.directory.size()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    fn add_journals(&self) {
        if self.test_passphrase().is_ok() {

        }
    }

    pub fn add_journal(&self, journal: String) {
        if let Some(encryption) = &self.encryption {
            let encrypted = encryption.encrypt(journal).unwrap();
            let path = self.directory.new_path().unwrap();
            fs::write(path, encrypted);
        }
    }

    pub fn has_passphrase(&self) -> bool {
        self.encryption.is_some()
    }

    pub fn no_passphrase(&self) -> bool {
        self.encryption.is_none()
    }

    pub fn set_passphrase(&mut self, passphrase: String) {
        self.encryption = Some(Encryption::new(passphrase))
    }

    pub fn test_passphrase(&self) -> Result<(), AppError> {
        if let Some(first_path) = self.directory.nth_path(0) {
            let decrypted = match (fs::read(&first_path), &self.encryption) {
                (Ok(encrypted), Some(encryption)) => {
                    encryption.decrypt(encrypted)
                },
                _ => return Ok(()),
            };
            if decrypted.is_err() {
                return Err(AppError::IncorrectPassword);
            }
        }
        Ok(())
    }

    pub fn new_journal(&self, journal: String) {
        if let Some(encryption) = &self.encryption {
            let path = self.directory.new_path().unwrap();
            fs::write(path, encryption.encrypt(journal).unwrap());
        }
    }

    pub fn entries(&self) -> Vec<String> {
        match self.directory.entries() {
            Ok(entry) => entry,
            _ => vec![],
        }
    }

    pub fn read(&self) {
        if let Some(encryption) = &self.encryption {
            self.directory.read(|path| {
                let string = fs::read(path).unwrap();
                println!("{}", encryption.decrypt(string).unwrap());
            });
        }
    }
}

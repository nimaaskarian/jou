use std::{fs::{self, remove_file}, io};

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
    pub fn new(args:Args) -> io::Result<Self> {
        let mut app = App {
            journals_to_add: args.add,
            encryption: encryption_from_option_passphrase(args.passphrase),
            directory: Directory::new(args.path).unwrap(),
        };
        app.add_journals()?;
        Ok(app)
    }

    pub fn change_password(&mut self, new_password: String) -> io::Result<()>{
        let new_encryption = Encryption::new(new_password);
        if let Some(encryption) = &self.encryption {
            self.directory.read(|path| {
                let string = fs::read(&path).unwrap();
                let decrypted = encryption.decrypt(string).unwrap();
                if let Ok(encrypted) = new_encryption.encrypt(decrypted) {
                    fs::write(&path, encrypted)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.directory.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn nth_content(&mut self, n: usize) -> String {
        if let Some(encryption) = &self.encryption {
            let path = self.directory.nth_path(n).unwrap();
            let encrypted = fs::read(path).unwrap();
            let decrypted =encryption.decrypt(encrypted);
            return decrypted.unwrap()
        }
        String::new()
    }

    pub fn add_journals(&mut self) -> io::Result<()>{
        if self.test_passphrase().is_ok() {
            for journal in &self.journals_to_add {
                self.add_journal(journal)?;
            }
        }
        Ok(())
    }

    pub fn add_journal<S: AsRef<str>>(&self, journal: S) -> io::Result<()>{
        if let Some(encryption) = &self.encryption {
            let encrypted = encryption.encrypt(journal).unwrap();
            let path = self.directory.new_path().unwrap();
            fs::write(path, encrypted)?;
        }
        Ok(())
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

    pub fn test_passphrase(&mut self) -> Result<(), AppError> {
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

    pub fn edit_nth(&mut self, n: usize, journal: String) -> io::Result<()>{
        if let (Some(path), Some(encryption)) = (self.directory.nth_path(n), &self.encryption){
            fs::write(path, encryption.encrypt(journal).unwrap())?;
        }
        Ok(())
    }

    pub fn delete_nth(&mut self, n: usize) -> io::Result<()>{
        if let Some(path) = self.directory.nth_path(n) {
            remove_file(path)?;
        }
        Ok(())
    }

    pub fn entries(&mut self) -> Vec<String> {
        match self.directory.entries() {
            Ok(entry) => entry,
            _ => vec![],
        }
    }

    pub fn read(&self) -> io::Result<()> {
        if let Some(encryption) = &self.encryption {
            self.directory.read(|path| {
                let encrypted = fs::read(path)?;
                if let Ok(decrypted) = encryption.decrypt(encrypted) {
                    println!("{}", decrypted);
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}

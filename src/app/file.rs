use std::{path::PathBuf, fs::{create_dir_all, self, File}, io, ffi::{OsString, OsStr}};
use std::convert::TryFrom;
mod date;

pub struct Directory {
    path: PathBuf,
    entries: Vec<String>,
    last_len: usize,
}
#[derive(Debug)]
pub enum DirectoryError {
    IsFile,
    CreationFailed,
}
type Error = DirectoryError;

impl TryFrom<&str> for Directory {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        let path = PathBuf::from(value);
        Directory::new(path)
    }
}

impl TryFrom<String> for Directory {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        Self::try_from(value.as_str())
    }
}

impl Directory {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            if create_dir_all(&path).is_err() {
                return Err(DirectoryError::CreationFailed);
            }
        }
        if path.is_file() {
            Err(DirectoryError::IsFile)
        } else {
            Ok(Directory {
                path,
                last_len: 0,
                entries: vec![],
            })
        }
    }

    pub fn new_path(&self) -> io::Result<PathBuf> {
        let filename = date::current_string();
        let path = self.path.join(filename);
        Ok(path)
    }
    
    pub fn len(&self) -> usize {
        let count = {
            let mut count = 0;
            for _ in fs::read_dir(&self.path).unwrap() {
                count+=1
            }
            count
        };

        count
    }

    pub fn update_entries(&mut self) -> io::Result<()>{
        self.entries = vec![];
        for entry in fs::read_dir(&self.path).unwrap() {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                self.entries.push(name.to_str().unwrap().to_string());
            }
        }
        self.entries.sort_by(|a, b| b.cmp(a));
        Ok(())
    }

    pub fn entries(&mut self) -> io::Result<Vec<String>> {
        let len = self.len();
        if len != self.last_len {
            self.update_entries();
            self.last_len = len;
        }
        Ok(self.entries.clone())
    }

    pub fn read<F>(&self, callback: F) -> io::Result<()> where 
        F: Fn(PathBuf) {
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            callback(entry.path());
        }
        Ok(())
    }

    pub fn nth_path(&mut self, n: usize) -> Option<PathBuf> {
        for (i, entry) in self.entries().unwrap().iter().enumerate() {
            if i == n {
                return Some(self.path.join(entry))
            }
        }
        None
    }
}

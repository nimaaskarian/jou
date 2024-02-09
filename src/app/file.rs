use std::{path::PathBuf, fs::{create_dir_all, self, File}, io, ffi::{OsString, OsStr}};
use std::convert::TryFrom;
mod date;

pub struct Directory {
    path: PathBuf,
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
            })
        }
    }

    pub fn new_file(&self) -> io::Result<File> {
        let path = self.new_path()?;
        // File::open(path)
        File::create(&path)
    }

    pub fn new_path(&self) -> io::Result<PathBuf> {
        let filename = date::current_string();
        let path = self.path.join(filename);
        // File::create(&path)?;
        Ok(path)
    }
    
    pub fn size(&self) -> usize {
        let count = {
            let mut count = 0;
            for _ in fs::read_dir(&self.path) {
                count+=1
            }
            count
        };

        count
    }

    pub fn read<F>(&self, callback: F) -> io::Result<()> where 
        F: Fn(PathBuf) {
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            callback(entry.path());
        }
        Ok(())
    }

    pub fn names(&self) -> Vec<OsString> {
        let mut output = vec![];
        let dir_iterator = match fs::read_dir(&self.path) {
            Ok(some) => some,
            Err(_) => return output,
        };
        for entry in dir_iterator {
            let entry = entry.unwrap();
            output.push(entry.file_name());
        }
        return output;
    }

    pub fn nth_path(&self, n: usize) -> Option<PathBuf> {
        let dir_iterator = match fs::read_dir(&self.path) {
            Ok(some) => some,
            Err(_) => return None,
        };
        for (i, entry) in dir_iterator.enumerate() {
            let entry = entry.unwrap();
            if i == n {
                return Some(entry.path())
            }
        }
        None
    }
    
    pub fn name(&self) -> Option<&OsStr> {
        self.path.file_name()
    }
}

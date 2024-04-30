use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub enum FileError {
    NotFound,
    IoError(io::Error),
}

pub(crate) struct ExistingFile(File);

impl ExistingFile {
    pub fn new(path: &str) -> Result<Self, FileError> {
        let file_path = Path::new(path);
        if file_path.exists() {
            match File::open(file_path) {
                Ok(file) => Ok(ExistingFile(file)),
                Err(err) => Err(FileError::IoError(err)),
            }
        } else {
            Err(FileError::NotFound)
        }
    }
   
    pub fn read_lines(&self) -> impl Iterator<Item = String>  + '_ {
        let buffered_reader = BufReader::new(&self.0);
        buffered_reader.lines().map(|line| line.unwrap())
    }

    
}

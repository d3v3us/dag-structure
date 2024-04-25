use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::io::prelude::*;
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

    pub fn read_data(&mut self) -> io::Result<String> {
        let mut data = String::new();
        self.0.read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn write_data(&mut self, data: &str) -> io::Result<()> {
        self.0.write_all(data.as_bytes())?;
        Ok(())
    }
   
    pub fn read_lines<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        let buffered_reader = BufReader::new(&self.0);
        let lines = buffered_reader.lines().map(|line| line.unwrap());
        lines
    }

    
}

use std::{
    fs::File,
    io::{self, BufReader},
};

use ropey::Rope;

#[derive(Debug, Default)]
pub struct Buffer {
    pub file_name: String,
    pub text: Rope,
}

impl Buffer {
    pub fn from_file(file: String) -> Result<Buffer, io::Error> {
        let text = Rope::from_reader(BufReader::new(File::open(file.clone())?))?;
        let buf = Buffer { file_name: file, text };
        Ok(buf)
    }
}

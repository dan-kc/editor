use std::{
    fs::File,
    io::{self, BufReader},
};

use ropey::Rope;

#[derive(Debug, Default)]
pub struct Buffer {
    pub file: String,
    pub text: Rope,
}

impl Buffer {
    pub fn from_file(file: String) -> Result<Buffer, io::Error> {
        let text = Rope::from_reader(BufReader::new(File::open(file.clone())?))?;
        let buf = Buffer { file, text };
        Ok(buf)
    }
}

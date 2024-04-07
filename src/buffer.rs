use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crop::{Rope, RopeBuilder};

trait FileReader {
    fn from_file(file: File) -> io::Result<Rope>;
}

impl FileReader for Rope {
    fn from_file(file: File) -> io::Result<Rope> {
        let reader = BufReader::new(file);
        let mut rope_builder = RopeBuilder::new();
        for line_result in reader.lines() {
            let line = line_result?;
            match std::str::from_utf8(line.as_bytes()) {
                Ok(utf8_str) => {
                    let mut str_new_line = utf8_str.to_string();
                    str_new_line.push('\n');
                    rope_builder.append(str_new_line);
                }
                Err(error) => {
                    let error_msg = format!("UTF-8 decoding error: {}", error);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, error_msg));
                }
            };
        }
        Ok(rope_builder.build())
    }
}

#[derive(Debug, Default)]
pub struct Buffer {
    pub file_name: String,
    pub rope: Rope,
}

impl Buffer {
    pub fn from_file(file_name: &str) -> io::Result<Buffer> {
        let rope = Rope::from_file(File::open(file_name)?)?;
        let buf = Buffer {
            file_name: file_name.to_string(),
            rope,
        };
        Ok(buf)
    }
}

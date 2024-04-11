use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::RangeBounds,
};

use crop::{Rope, RopeBuilder};

use crate::app::Cursor;

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
        rope_builder.append("EOF");
        Ok(rope_builder.build())
    }
}

#[derive(Debug, Default)]
pub struct Buffer {
    file_name: String,
    rope: Rope,
}

impl Buffer {
    // Create text buffer from file
    pub fn from_file(file_name: &str) -> io::Result<Buffer> {
        let rope = Rope::from_file(File::open(file_name)?)?;
        let buf = Buffer {
            file_name: file_name.to_string(),
            rope,
        };
        Ok(buf)
    }
    pub fn file_name(&self) -> &String {
        &self.file_name
    }
    /// Number of lines in buffer.
    pub fn len_lines(&self) -> usize {
        self.rope.line_len() - 1
    }
    // Rope line as a string.
    pub fn line(&self, idx: usize) -> String {
        self.rope.line(idx).to_string()
    }
    // Line col segment for line.
    pub fn numb_col(&self, line_idx: usize) -> String {
        let mut res: String = " ".to_string();
        let total_width = self.line_numb_col_width();
        let line_numb = &(line_idx + 1).to_string();
        res.push_str(line_numb);
        let right_padding_width = total_width - 3 - line_numb.len();
        for _ in 0..right_padding_width {
            res.push(' ');
        }
        res.push('┆');
        res.push(' ');
        res
    }
    /// Byte index of first byte in line.
    pub fn byte_idx_of_line_start(&self, idx: usize) -> usize {
        self.rope.byte_of_line(idx)
    }
    /// Byte index of last byte in line.
    pub fn byte_idx_of_line_end(&self, idx: usize) -> usize {
        self.rope.byte_of_line(idx + 1) - 1
    }
    /// Byte index of line with char_offset
    pub fn byte_idx_of_char(&self, line_idx: usize, char_offset: usize) -> Option<usize> {
        let line = self.line(line_idx);
        if char_offset >= line.len() {
            return None;
        }

        let mut byte_idx = 0;
        for (i, c) in line.chars().enumerate() {
            if i == char_offset {
                return Some(byte_idx);
            }
            byte_idx += c.len_utf8();
        }

        None
    }
    /// Byte index of cursor position
    pub fn byte_idx_of_cursor_pos(&self, cursor: &Cursor) -> Option<usize> {
        self.byte_idx_of_char(cursor.y(), cursor.x())
    }
    pub fn remove<R: RangeBounds<usize>>(&mut self, byte_range: R) {
        self.rope.delete(byte_range)
    }
    pub fn remove_line(&mut self, line_idx: usize) {
        self.remove(self.byte_idx_of_line_start(line_idx)..=self.byte_idx_of_line_end(line_idx))
    }
    pub fn insert(&mut self, byte_offset: usize, text: String) {
        self.rope.insert(byte_offset, text)
    }
    /// Calculates the with of number column
    /// left margin + right margin + border = 3
    pub fn line_numb_col_width(&self) -> usize {
        self.len_lines().to_string().len() + 3
    }
}

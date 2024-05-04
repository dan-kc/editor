use crop::{Rope, RopeBuilder, RopeSlice};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::RangeBounds,
    panic,
};

#[derive(Debug, Default)]
pub struct Buffer {
    file_name: Box<str>,
    rope: Rope,
    cursor: Cursor,
}
impl Buffer {
    // Create text buffer from file
    pub fn from_file(file_name: String) -> io::Result<Buffer> {
        let file_name: Box<str> = file_name.into();
        let file = File::open(file_name.as_ref())?;
        let rope = Rope::from_file(file)?;
        let buf = Buffer {
            file_name,
            rope,
            cursor: Default::default(),
        };
        Ok(buf)
    }
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }
    /// Number of lines in buffer.
    pub fn lines_count(&self) -> usize {
        self.rope.line_len()
    }
    /// Number of chars in line INCLUDING '\n'
    pub fn line_chars_count(&self, line_idx: usize) -> usize {
        self.rope.line(line_idx).chars().count() + 1
    }
    pub fn byte(&self, idx: usize) -> u8 {
        self.rope.byte(idx)
    }
    /// Panics if line does not exist. The \n is NOT included in line.
    pub fn line(&self, idx: usize) -> RopeSlice {
        self.rope.line(idx)
    }
    pub fn is_empty(&self) -> bool {
        self.rope.is_empty()
    }
    /// Byte index of first byte in line.
    pub fn byte_idx_of_line_start(&self, line_idx: usize) -> usize {
        self.rope.byte_of_line(line_idx)
    }
    /// Byte index of last byte in line. usually \n
    pub fn byte_idx_of_line_end(&self, line_idx: usize) -> usize {
        let result = panic::catch_unwind(|| self.rope.byte_of_line(line_idx + 1));
        match result {
            Ok(r) => r - 1,
            Err(_) => {
                let is_last_line = self.lines_count() == line_idx + 1;
                if is_last_line {
                    self.rope.byte_len() - 1
                } else {
                    panic!("line out of bounds");
                }
            }
        }
    }
    pub fn byte_idx_under_cursor(&self) -> Option<usize> {
        let cursor = self.cursor;
        let line = self.line(cursor.y);
        let mut byte_idx = 0;
        for (i, c) in line.chars().enumerate() {
            if i == cursor.x {
                return Some(byte_idx);
            }
            byte_idx += c.len_utf8();
        }
        let is_on_newline_char = self.line_chars_count(cursor.y) == self.cursor.x + 1;
        if !is_on_newline_char {
            return None;
        };
        if self.lines_count() == cursor.y + 1 {
            None
        } else {
            Some(byte_idx)
        }
    }
    pub fn char_under_cursor(&self) -> Option<char> {
        if !self.in_rope_bounds() {
            return None;
        };

        let line_idx = self.cursor().y;
        let x_offset = self.cursor().x;
        let mut line = self.line(line_idx).to_string();
        line.push('\n');
        let chars = line.chars().enumerate();

        for (idx, char) in chars {
            if x_offset == idx {
                return Some(char);
            }
        }

        None
    }
    /// Is the cursor currently in bounds of the rope. \n is in bounds
    pub fn in_rope_bounds(&self) -> bool {
        self.byte_idx_under_cursor().is_some()
    }
    /// Is the cursor currently in bounds of line. \n is out of bounds
    pub fn in_bounds(&self) -> bool {
        match self.char_under_cursor() {
            Some(char) => char != '\n',
            None => false,
        }
    }
    pub fn numb_col(&self, line_idx: usize) -> Box<str> {
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
        res.into_boxed_str()
    }
    pub fn line_empty(&self, line_idx: usize) -> bool {
        self.line(line_idx).chars().count() == 0
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
        self.lines_count().to_string().len() + 3
    }
    pub fn words<T: RangeBounds<usize>>(&self, byte_range: T) -> Box<[Word]> {
        let mut byte_idx = match byte_range.start_bound() {
            std::ops::Bound::Included(val) => *val,
            std::ops::Bound::Excluded(val) => *val + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let rope_slice = self.rope.byte_slice(byte_range);
        let chars = rope_slice.chars();
        let mut words = Vec::new();
        let mut curr_chars = Vec::new();
        let mut char_idx = self.cursor().x ;
        let mut last_len: Option<usize> = None;

        for char in chars {
            let char = Char {
                char,
                byte_idx,
                char_idx,
            };
            char_idx += 1;
            byte_idx += char.char.len_utf8();
            last_len = Some(char.char.len_utf8());

            let curr_type = char.classify();
            let prev_type = curr_chars.last().map(|char: &Char| char.classify());

            let same_type_as_prev = match prev_type {
                Some(t) => t == curr_type,
                None => false,
            };
            let is_empty = curr_chars.is_empty();
            if same_type_as_prev {
                curr_chars.push(char);
                continue;
            };
            // types differ from this point
            if is_empty && curr_type == CharType::Whitespace {
                continue;
            }
            if is_empty && curr_type != CharType::Whitespace {
                curr_chars.push(char);
                continue;
            }
            // curr_chars non empty at this point
            let byte_len = curr_chars
                .iter()
                .fold(0, |acc, char| acc + char.char.len_utf8());
            let word = Word {
                chars: curr_chars.into_boxed_slice(),
                byte_len,
            };
            if curr_type == CharType::Whitespace {
                words.push(word);
                curr_chars = Vec::new();
            } else {
                words.push(word);
                curr_chars = vec![char];
            };
        }
        if !curr_chars.is_empty() && last_len.is_some() {
            let byte_len = curr_chars
                .iter()
                .fold(0, |acc, char| acc + char.char.len_utf8());
            let word = Word {
                chars: curr_chars.into_boxed_slice(),
                byte_len,
            };
            words.push(word);
        };
        words.into_boxed_slice()
    }
    pub fn words_long<T: RangeBounds<usize>>(&self, byte_range: T) -> Box<[Word]> {
        let mut byte_idx = match byte_range.start_bound() {
            std::ops::Bound::Included(val) => *val,
            std::ops::Bound::Excluded(val) => *val + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let rope_slice = self.rope.byte_slice(byte_range);
        let chars = rope_slice.chars();
        let mut words = Vec::new();
        let mut curr_chars = Vec::new();
        let mut char_idx = self.cursor().x;
        let mut last_len: Option<usize> = None;

        for char in chars {
            let char = Char {
                char,
                byte_idx,
                char_idx,
            };
            char_idx += 1;
            byte_idx += char.char.len_utf8();
            last_len = Some(char.char.len_utf8());

            let is_empty = curr_chars.is_empty();
            if !char.char.is_whitespace() {
                curr_chars.push(char);
                continue;
            }
            if is_empty {
                continue;
            };

            let byte_len = curr_chars
                .iter()
                .fold(0, |acc, char| acc + char.char.len_utf8());
            let word = Word {
                chars: curr_chars.into_boxed_slice(),
                byte_len,
            };
            words.push(word);
            curr_chars = Vec::new();
        }
        if !curr_chars.is_empty() && last_len.is_some() {
            let byte_len = curr_chars
                .iter()
                .fold(0, |acc, char| acc + char.char.len_utf8());
            let word = Word {
                chars: curr_chars.into_boxed_slice(),
                byte_len,
            };
            words.push(word);
        };
        words.into_boxed_slice()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum CharType {
    Letter,
    Number,
    Punctuation,
    Whitespace,
}

#[derive(Debug)]
pub struct Word {
    pub chars: Box<[Char]>,
    pub byte_len: usize,
}

#[derive(Debug)]
pub struct Char {
    pub char: char,
    pub byte_idx: usize, // of entire rope
    pub char_idx: usize, // of specified range
}

impl Char {
    fn classify(&self) -> CharType {
        let c = self.char;
        if c.is_whitespace() {
            CharType::Whitespace
        } else if c.is_ascii_punctuation() {
            CharType::Punctuation
        } else if c.is_numeric() {
            CharType::Number
        } else {
            CharType::Letter
        }
    }
}

trait FileReader {
    fn from_file(file: File) -> io::Result<Rope>;
}
impl FileReader for Rope {
    fn from_file(file: File) -> io::Result<Rope> {
        let reader = BufReader::new(&file);
        let mut rope_builder = RopeBuilder::new();

        let mut lines = reader.lines().peekable();
        while let Some(line_result) = lines.next() {
            let line = line_result?;
            match std::str::from_utf8(line.as_bytes()) {
                Ok(utf8_str) => {
                    let mut str_new_line = utf8_str.to_string();
                    if lines.peek().is_some() {
                        str_new_line.push('\n');
                    };
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

#[derive(Debug, Default, Copy, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    enum MockFile {
        Basic,
        Empty,
        Sparse,
    }

    fn init(file: MockFile) -> Buffer {
        let file_name: String = match file {
            MockFile::Basic => "tests/mocks/basic.txt".into(),
            MockFile::Empty => "tests/mocks/empty.txt".into(),
            MockFile::Sparse => "tests/mocks/sparse.txt".into(),
        };
        Buffer::from_file(file_name).unwrap()
    }

    #[test]
    fn test_open_file() {
        let buffer = init(MockFile::Sparse);
        assert_eq!(buffer.byte(0), b'P');
        assert_eq!(buffer.byte(3), b'\n');
        assert_eq!(buffer.byte(4), b'\n');
        assert_eq!(buffer.byte(5), b'\n');

        assert_eq!(buffer.byte(14), b' ');
        assert_eq!(buffer.byte(27), b'.');

        let result = panic::catch_unwind(|| buffer.byte(28));
        assert!(result.is_err());
    }

    #[test]
    fn test_open_empty_file() {
        let buffer = init(MockFile::Empty);
        assert!(buffer.is_empty())
    }
    #[test]
    fn test_byte_idx_under_cursor() {
        let mut buffer = init(MockFile::Basic);

        buffer.cursor_mut().y = 0;
        buffer.cursor_mut().x = 10;
        assert_eq!(buffer.byte_idx_under_cursor().unwrap(), 12);

        buffer.cursor_mut().y = 0;
        buffer.cursor_mut().x = 34;
        assert_eq!(buffer.byte_idx_under_cursor().unwrap(), 37); // \n
    }
    #[test]
    fn test_char_under_cursor() {
        let mut buffer = init(MockFile::Basic);

        buffer.cursor_mut().y = 3;
        buffer.cursor_mut().x = 18;
        assert_eq!(buffer.char_under_cursor().unwrap(), 'd');

        buffer.cursor_mut().y = 0;
        buffer.cursor_mut().x = 34;
        assert_eq!(buffer.char_under_cursor().unwrap(), '\n');

        buffer.cursor_mut().y = 2;
        buffer.cursor_mut().x = 0;
        assert_eq!(buffer.char_under_cursor().unwrap(), '\n');
    }
    #[test]
    fn test_fail_char_under_cursor() {
        let mut buffer = init(MockFile::Basic);

        buffer.cursor_mut().y = 0;
        buffer.cursor_mut().x = 35;
        assert!(buffer.char_under_cursor().is_none());

        buffer.cursor_mut().y = 2;
        buffer.cursor_mut().x = 1;
        assert!(buffer.char_under_cursor().is_none());

        buffer.cursor_mut().y = 3;
        buffer.cursor_mut().x = 34;
        assert!(buffer.char_under_cursor().is_none());
    }
    // #[test]
    // fn test_x_char_offset() {
    //     let mut buffer = init(MockFile::Basic);
    // }
    // #[test]
    // fn test_fail_x_char_offset() {
    //     todo!()
    // }
}

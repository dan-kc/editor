use ropey::{Rope, RopeSlice};
use std::{fmt::Display, fs::File, io, ops::RangeBounds, result::Result};

// TODO: account for a change in file name.
#[derive(Debug, Default)]
pub struct Buffer {
    file_name: Box<str>,
    rope: Rope,
    pub cursor: Cursor,
}

impl Buffer {
    // Create buffer from file
    pub fn from_file(file_name: String) -> io::Result<Buffer> {
        let file_name: Box<str> = file_name.into();
        let file = File::open(file_name.as_ref())?;
        let rope = Rope::from_reader(file)?.remove_last_new_line_char();
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

    /// Number of lines in buffer
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// The line as a RopeSlice.
    /// The \n is included in each line.
    pub fn line(&self, line_idx: usize) -> BufferResult<Line> {
        let in_bounds = line_idx < self.len_lines();
        if !in_bounds {
            return Err(Error::LineIndexOutOfBounds {
                attempted_idx: line_idx,
                len_lines: self.len_lines(),
            });
        }

        Ok(self.rope.line(line_idx).into())
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// char len of rope.
    pub fn len(&self) -> usize {
        self.rope.len_chars()
    }

    /// Is the position on the tail of the rope. The tail is the position just after the last element
    /// in the rope.
    pub fn on_rope_tail(&self, pos: (usize, usize)) -> bool {
        let on_last_line = self.len_lines() == pos.1 + 1;
        if !on_last_line {
            return false;
        }

        // We are on the last line
        self.line(pos.1).unwrap().len() == pos.0
    }

    pub fn char_idx_under_pos(
        &self,
        pos: (usize, usize),
    ) -> BufferResult<usize> {
        if !self.in_rope_bounds(pos) {
            return Err(Error::CursorOutOfBounds);
        };

        let line_idx = pos.1;
        let x_offset = pos.0;

        Ok(self.rope.try_line_to_char(line_idx)? + x_offset)
    }

    /// Returns the char under the position.
    pub fn char_under_pos(&self, pos: (usize, usize)) -> BufferResult<char> {
        let char_idx = self.char_idx_under_pos(pos)?;
        Ok(self.rope.char(char_idx))
    }

    /// Is the position currently in bounds of the rope. \n classifies as is in bounds.
    pub fn in_rope_bounds(&self, pos: (usize, usize)) -> bool {
        let line = match self.line(pos.1) {
            Ok(l) => l,
            Err(_) => {
                return false;
            }
        };

        pos.0 < line.len()
    }

    /// Is the position currently in visual bounds. \n classifies as out of bounds
    pub fn in_visual_bounds(&self, pos: (usize, usize)) -> bool {
        let line = match self.line(pos.1) {
            Ok(l) => l,
            Err(_) => {
                return false;
            }
        };

        pos.0 < line.visual_len()
    }

    /// Number col for the line.
    pub fn numb_col(&self, line_idx: usize) -> Box<str> {
        let mut res = " ".to_string();
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

    /// Remove a range.
    /// Panics if the start of the range is greater than the end, or if the
    /// end is out of bounds (i.e. `end > len_chars()`).
    pub fn remove<R: RangeBounds<usize>>(
        &mut self,
        char_range: R,
    ) -> BufferResult<()> {
        self.rope.try_remove(char_range)?;
        Ok(())
    }

    pub fn char_idx_line_start(&self, line_idx: usize) -> BufferResult<usize> {
        let idx = self.rope.try_line_to_char(line_idx)?;
        Ok(idx)
    }

    /// Including '\n'.
    pub fn char_idx_line_end(&self, line_idx: usize) -> BufferResult<usize> {
        let idx = self.rope.try_line_to_char(line_idx + 1)? - 1;
        Ok(idx)
    }

    /// Delete line.
    pub fn delete_line(&mut self, line_idx: usize) -> BufferResult<()> {
        self.remove(
            self.char_idx_line_start(line_idx)?
                ..=self.char_idx_line_end(line_idx)?,
        )
    }

    pub fn insert<T: AsRef<str>>(
        &mut self,
        char_offset: usize,
        text: T,
    ) -> BufferResult<()> {
        self.rope.try_insert(char_offset, text.as_ref())?;

        Ok(())
    }

    /// Calculates the with of number column
    /// left margin + right margin + border = 3
    pub fn line_numb_col_width(&self) -> usize {
        self.len_lines().to_string().len() + 3
    }

    /// Returns the words of the line.
    /// # Panics
    ///
    /// Panics if the start of the range is greater than the end, or if the
    /// end is out of bounds (i.e. `end > len_chars()`).
    pub fn words<T: RangeBounds<usize>>(
        &self,
        char_range: T,
    ) -> BufferResult<Box<[Word]>> {
        let slice = self.rope.slice(char_range);
        let chars = slice.chars();
        let mut words = Vec::<Word>::new();
        let mut curr_chars = Vec::<Char>::new();

        for (char_idx, char) in chars.enumerate() {
            let char = Char { char, char_idx };
            let curr_type = char.classify();
            let prev_type = curr_chars.last().map(|char| char.classify());

            let same_type_as_prev = match prev_type {
                Some(t) => t == curr_type,
                None => false,
            };
            if same_type_as_prev {
                curr_chars.push(char);
                continue;
            };

            let is_empty = curr_chars.is_empty();
            if is_empty && curr_type == CharType::Whitespace {
                continue;
            }

            if is_empty && curr_type != CharType::Whitespace {
                curr_chars.push(char);
                continue;
            }

            let word = curr_chars.into_boxed_slice();

            if curr_type == CharType::Whitespace {
                words.push(word);
                curr_chars = Vec::new();
            } else {
                words.push(word);
                curr_chars = vec![char];
            };
        }

        if !curr_chars.is_empty() {
            let word = curr_chars.into_boxed_slice();

            words.push(word);
        };

        Ok(words.into_boxed_slice())
    }

    /// Returns the WORDS in the supplied byte_range
    pub fn words_long<T: RangeBounds<usize>>(
        &self,
        char_range: T,
    ) -> BufferResult<Box<[Word]>> {
        let slice = self.rope.slice(char_range);
        let chars = slice.chars();
        let mut words = Vec::<Word>::new();
        let mut curr_chars = Vec::new();
        let mut last_char_idx = 0;

        for (char_idx, char) in chars.enumerate() {
            let char = Char { char, char_idx };
            if char.classify() != CharType::Whitespace {
                curr_chars.push(char);
                continue;
            }

            let is_empty = curr_chars.is_empty();
            if is_empty {
                continue;
            };

            let word = curr_chars.into_boxed_slice();

            words.push(word);
            curr_chars = Vec::new();
        }

        if !curr_chars.is_empty() {
            let word = curr_chars.into_boxed_slice();

            words.push(word);
        };

        Ok(words.into_boxed_slice())
    }

    /// Returns the (x,y) position of the last visual char in file.
    /// Should error if no chars in file.
    pub fn end_pos(&self) -> BufferResult<(usize, usize)> {
        if self.rope.len_chars() == 0 {
            return Err(Error::NoCharsInFile);
        };
        let last_line_idx = self.rope.len_lines() - 1;
        let last_line = self.line(last_line_idx)?;

        if last_line.is_empty() {
            let prev_line_idx = last_line_idx - 1;
            let prev_line = self
                .line(last_line_idx - 1)
                .expect("prev line should exist");
            Ok((prev_line.len() - 2, prev_line_idx))
        } else {
            Ok((last_line.len() - 1, last_line_idx))
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NoCharsInFile,
    NoBytesInLine,
    CursorOutOfBounds,
    CharRangeInvalid {
        start: usize,
        end: usize,
    },
    CharRangeOutOfBounds {
        start: Option<usize>,
        end: Option<usize>,
        rope_char_len: usize,
    },
    InsertPointOutOfBounds {
        attempted_idx: usize,
        rope_char_count: usize,
    },
    LineIndexOutOfBounds {
        attempted_idx: usize,
        len_lines: usize,
    },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Buffer error: ");
        match *self {
            Self::NoCharsInFile => {
                write!(f, "no bytes in line")
            }
            Self::NoBytesInLine => {
                write!(f, "no bytes in line")
            }
            Self::CursorOutOfBounds => {
                write!(f, "cursor is out of bounds")
            }
            Self::InsertPointOutOfBounds {
                attempted_idx,
                rope_char_count,
            } => {
                write!(
                    f,
                    "Insert index {} out of bounds. {} chars in rope.",
                    attempted_idx, rope_char_count
                )
            }
            Self::CharRangeOutOfBounds {
                start: start_idx_opt,
                end: end_idx_opt,
                rope_char_len: len,
            } => {
                write!(f, "Char range out of bounds: char range ")?;
                write_range(f, start_idx_opt, end_idx_opt)?;
                write!(f, ", Rope/RopeSlice char length {}", len)
            }
            Self::CharRangeInvalid {
                start: start_idx,
                end: end_idx,
            } => {
                write!(
                    f,
                    "Invalid char range {}..{}: start must be <= end",
                    start_idx, end_idx
                )
            }
            Self::LineIndexOutOfBounds {
                attempted_idx,
                len_lines: rope_len,
            } => {
                write!(
                    f,
                    "Line index {} out of bounds. {} lines in rope.",
                    attempted_idx, rope_len
                )
            }
        }
    }
}

impl From<ropey::Error> for Error {
    fn from(err: ropey::Error) -> Self {
        match err {
            ropey::Error::CharRangeInvalid(start, end) => {
                todo!()
            }
            ropey::Error::CharIndexOutOfBounds(start, end) => {
                todo!()
            }
            ropey::Error::LineIndexOutOfBounds(start, end) => {
                todo!()
            }
            _ => {
                panic!("this is a rope error we have not accounted for")
            }
        }
    }
}

pub type BufferResult<T> = Result<T, Error>;

#[derive(Eq, PartialEq, Copy, Clone)]
enum CharType {
    Letter,
    Number,
    Punctuation,
    Whitespace,
}

type Word = Box<[Char]>;

#[derive(Debug)]
pub struct Char {
    pub char: char,
    pub char_idx: usize,
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

fn write_range(
    f: &mut std::fmt::Formatter<'_>,
    start_idx: Option<usize>,
    end_idx: Option<usize>,
) -> std::fmt::Result {
    match (start_idx, end_idx) {
        (None, None) => {
            write!(f, "..")
        }

        (Some(start), None) => {
            write!(f, "{}..", start)
        }

        (None, Some(end)) => {
            write!(f, "..{}", end)
        }

        (Some(start), Some(end)) => {
            write!(f, "{}..{}", start, end)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Cursor {
    fn from(value: (usize, usize)) -> Self {
        Cursor {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<(usize, usize)> for Cursor {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

/// A ropeslice wrapper with extra methods for line operations.
#[derive(Debug)]
pub struct Line<'a> {
    slice: RopeSlice<'a>,
}

impl<'a> From<RopeSlice<'a>> for Line<'a> {
    fn from(slice: RopeSlice<'a>) -> Self {
        Self { slice }
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the underlying rope. Swap out newline char, and whitespace char, for fonts.
        write!(f, "{}", self.slice)
    }
}

impl<'a> Line<'a> {
    /// Returns true if there are no visual chars in line. This means the underlying rope is empty
    /// or it only has a newline char.
    pub fn is_visually_empty(&self) -> bool {
        let chars = self.slice.chars();
        let len = chars.len();
        if len == 0 || len == 1 && chars.last().unwrap() == '\n' {
            return true;
        };

        false
    }
    /// Returns true if there are no chars in line. This means the underlying rope is empty.
    pub fn is_empty(&self) -> bool {
        self.slice.len_chars() == 0
    }
    /// Returns the char length of the line, ignoring newline chars.
    pub fn visual_len(&self) -> usize {
        // Check last char,
        let chars = self.chars();
        let last_char = match chars.last() {
            Some(char) => char,
            None => return 0,
        };

        match last_char {
            '\n' => chars.len() - 1,
            _ => chars.len(),
        }
    }
    /// Returns the char length of the line, including newline chars.
    pub fn len(&self) -> usize {
        self.slice.len_chars()
    }
    /// Returns an array of words.
    pub fn words(&self) -> Box<[Word]> {
        todo!()
    }
    /// Returns an array of long words seperated by whitespaces.
    pub fn words_long(&self) -> Box<[Word]> {
        todo!()
    }
    /// Returns the chars including newline char.
    pub fn chars(&self) -> Box<[char]> {
        let chars = self.slice.chars();
        let mut res = vec![];
        for (char_idx, char) in chars.enumerate() {
            res.push(char);
        }
        res.into_boxed_slice()
    }
    /// Returns the chars excluding the newline char.
    pub fn visual_chars(&self) -> Box<[char]> {
        todo!()
    }
}

trait LastLineRemover {
    fn remove_last_new_line_char(self) -> Self;
}

impl LastLineRemover for Rope {
    /// From reader adds an extra '\n'sometimes.
    fn remove_last_new_line_char(mut self) -> Self {
        let is_empty = self.len_chars() == 0;
        if !is_empty {
            let last_char_idx = self.len_chars() - 1;
            self.remove(last_char_idx..=last_char_idx);
        };
        self
    }
}

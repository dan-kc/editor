use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};
use ratatui::style::{Color, Style};
use std::{
    char,
    error::{self, Error},
    fmt::Display,
};

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
    buffer: Buffer,
    logger: Logger,
    mode: Mode,
    messages: Vec<Notification>,
}

impl App {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            ..Default::default()
        }
    }
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
    pub fn mode_mut(&mut self) -> &mut Mode {
        &mut self.mode
    }
    pub fn logger(&self) -> &Logger {
        &self.logger
    }
    pub fn logger_mut(&mut self) -> &mut Logger {
        &mut self.logger
    }
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }
    pub fn running_state_mut(&mut self) -> &mut RunningState {
        &mut self.running_state
    }
    pub fn messages(&self) -> &[Notification] {
        self.messages.as_slice()
    }
    pub fn push_msg(&mut self, msg: Notification) {
        self.messages.push(msg)
    }
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    pub fn quit(&mut self) {
        self.running_state = RunningState::Done
    }
    pub fn insert_char(&mut self, char: char) -> SoftResult<()> {
        let byte_idx = self
            .buffer()
            .byte_idx_under_cursor()
            .expect("out of bounds");
        self.buffer.insert(byte_idx, char.to_string());
        self.move_right();
        Ok(())
    }
    pub fn delete_line(&mut self) -> SoftResult<()> {
        if self.buffer().lines_count() == 0 {
            return Err(SoftError::NoLinesLeft);
        }
        let line_idx = self.buffer().cursor().y;
        self.buffer.remove_line(line_idx);
        Ok(())
    }
    pub fn move_up(&mut self) -> SoftResult<()> {
        self.buffer()
            .cursor()
            .y
            .checked_sub(1)
            .ok_or(SoftError::OutOfBounds(Direction::Up))
            .map(|y_pos| {
                self.buffer_mut().cursor_mut().y = y_pos;
            })
    }
    pub fn move_down(&mut self) -> SoftResult<()> {
        let cursor_on_last_line = self.buffer.lines_count() == self.buffer().cursor().y + 1;
        if cursor_on_last_line {
            return Err(SoftError::OutOfBounds(Direction::Down));
        };
        let y_pos = self
            .buffer()
            .cursor()
            .y
            .checked_add(1)
            .expect("y cursor value overflow");
        self.buffer_mut().cursor_mut().y = y_pos;
        Ok(())
    }
    pub fn move_left(&mut self) -> SoftResult<()> {
        self.buffer()
            .cursor()
            .x
            .checked_sub(1)
            .ok_or(SoftError::OutOfBounds(Direction::Left))
            .map(|x_pos| {
                self.buffer_mut().cursor_mut().x = x_pos;
            })
    }
    pub fn move_right(&mut self) {
        let x_pos = self
            .buffer()
            .cursor()
            .x
            .checked_add(1)
            .expect("x cursor value overflow");

        self.buffer_mut().cursor_mut().x = x_pos;
    }
    pub fn move_to_top(&mut self) -> SoftResult<()> {
        if self.buffer().cursor().y == 0 {
            return Err(SoftError::AlreadyAtTop);
        }
        self.buffer_mut().cursor_mut().y = 0;
        Ok(())
    }
    pub fn move_to_bottom(&mut self) -> SoftResult<()> {
        if self.buffer().is_empty() {
            return Err(SoftError::BufferEmpty);
        }
        if self.buffer().cursor().y + 1 == self.buffer().lines_count() {
            return Err(SoftError::AlreadyAtBottom);
        }
        self.buffer_mut().cursor_mut().y = self.buffer.lines_count() - 1;
        Ok(())
    }
    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
    pub fn move_next_word_start(&mut self) -> SoftResult<()> {
        if !self.buffer().in_bounds() {
            return Err(SoftError::CursorOutOfBounds);
        };
        let start = match self.buffer().byte_idx_under_cursor() {
            Some(s) => s,
            None => return Err(SoftError::CursorOutOfBounds),
        };
        let end = self.buffer().byte_idx_of_line_end(self.buffer().cursor().y);
        let words = self.buffer().words(start..end);
        let word_idx = match self.buffer().char_under_cursor() {
            Some(v) => {
                if v.is_whitespace() {
                    0
                } else {
                    1
                }
            }
            None => return Err(SoftError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(SoftError::NoMoreWordsInLine),
            Some(word) => {
                let first_char = word
                    .chars
                    .first()
                    .expect("Chars array for next word is empty");
                self.buffer_mut().cursor_mut().x = first_char.char_idx;
                Ok(())
            }
        }
    }
    pub fn move_next_word_start_long(&mut self) -> SoftResult<()> {
        if !self.buffer().in_bounds() {
            return Err(SoftError::CursorOutOfBounds);
        };
        let start = match self.buffer().byte_idx_under_cursor() {
            Some(s) => s,
            None => return Err(SoftError::CursorOutOfBounds),
        };
        let end = self.buffer().byte_idx_of_line_end(self.buffer().cursor().y);
        let words = self.buffer().words_long(start..end);
        let word_idx = match self.buffer().char_under_cursor() {
            Some(v) => {
                if v.is_whitespace() {
                    0
                } else {
                    1
                }
            }
            None => return Err(SoftError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(SoftError::NoMoreWordsInLine),
            Some(word) => {
                let last_char = word
                    .chars
                    .first()
                    .expect("Chars array for next word is empty");
                self.buffer_mut().cursor_mut().x = last_char.char_idx;
                Ok(())
            }
        }
    }
    pub fn move_next_word_end(&mut self) -> SoftResult<()> {
        if !self.buffer().in_bounds() {
            return Err(SoftError::CursorOutOfBounds);
        };
        let start = match self.buffer().byte_idx_under_cursor() {
            Some(s) => s,
            None => return Err(SoftError::CursorOutOfBounds),
        };
        let end = self.buffer().byte_idx_of_line_end(self.buffer().cursor().y);
        let words = self.buffer().words(start..end);
        let word_idx = match words.first() {
            Some(word) => {
                if word.chars.len() == 1 {
                    1
                } else {
                    0
                }
            }
            None => return Err(SoftError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(SoftError::NoMoreWordsInLine),
            Some(word) => {
                let last_char = word
                    .chars
                    .last()
                    .expect("Chars array for next word is empty");
                self.buffer_mut().cursor_mut().x = last_char.char_idx;
                Ok(())
            }
        }
    }
    pub fn move_next_word_end_long(&mut self) -> SoftResult<()> {
        if !self.buffer().in_bounds() {
            return Err(SoftError::CursorOutOfBounds);
        };
        let start = match self.buffer().byte_idx_under_cursor() {
            Some(s) => s,
            None => return Err(SoftError::CursorOutOfBounds),
        };
        let end = self.buffer().byte_idx_of_line_end(self.buffer().cursor().y);
        let words = self.buffer().words_long(start..end);
        let word_idx = match words.first() {
            Some(word) => {
                if word.chars.len() == 1 {
                    1
                } else {
                    0
                }
            }
            None => return Err(SoftError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(SoftError::NoMoreWordsInLine),
            Some(word) => {
                let last_char = word
                    .chars
                    .last()
                    .expect("Chars array for next word is empty");
                self.buffer_mut().cursor_mut().x = last_char.char_idx;
                Ok(())
            }
        }
    }
    pub fn move_start_line(&mut self) -> SoftResult<()> {
        let line_idx = self.buffer().cursor().y;
        if self.buffer().line_empty(line_idx) {
            self.buffer_mut().cursor_mut().x = 0;
            return Err(SoftError::NoCharactersInLine);
        };
        if self.buffer().cursor().x == 0 {
            return Err(SoftError::AlreadyAtLineStart);
        }

        self.buffer_mut().cursor_mut().x = 0;
        Ok(())
    }
    /// Move to the last non-newline char of the current line. If there are no chars, then move to (0,0).
    pub fn move_end_line(&mut self) -> SoftResult<()> {
        let line_idx = self.buffer().cursor().y;
        if self.buffer().line_empty(line_idx) {
            self.buffer_mut().cursor_mut().x = 0;
            return Err(SoftError::NoCharactersInLine);
        };
        let char_count = self.buffer().line(line_idx).chars().count();
        if self.buffer().cursor().x == char_count - 1 {
            return Err(SoftError::AlreadyAtLineEnd);
        }

        self.buffer_mut().cursor_mut().x = char_count - 1;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

/// Soft errors that are displayed in the status line
#[derive(Debug)]
pub enum SoftError {
    OutOfBounds(Direction),
    NoLinesLeft,
    KeyUnmapped,
    AlreadyAtTop,
    AlreadyAtBottom,
    NoMoreWordsInLine,
    AlreadyAtLineStart,
    AlreadyAtLineEnd,
    NoCharactersInLine,
    LineDoesNotExist,
    CursorOutOfBounds,
    BufferEmpty,
}
impl Error for SoftError {}
impl Display for SoftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds(dir) => {
                write!(f, "Out of bounds: can't move {}", dir)
            }
            Self::NoLinesLeft => {
                write!(f, "No lines to delete")
            }
            Self::KeyUnmapped => {
                write!(f, "Key is unmapped")
            }
            Self::AlreadyAtTop => {
                write!(f, "Already at top of file")
            }
            Self::AlreadyAtBottom => {
                write!(f, "Already at bottom of file")
            }
            Self::NoMoreWordsInLine => {
                write!(f, "No more words to jump to")
            }
            Self::AlreadyAtLineStart => {
                write!(f, "Already at line start")
            }
            Self::AlreadyAtLineEnd => {
                write!(f, "Already at line end")
            }
            Self::NoCharactersInLine => {
                write!(f, "No characters in line")
            }
            Self::LineDoesNotExist => {
                write!(f, "Line does no exist")
            }
            Self::CursorOutOfBounds => {
                write!(f, "Cursor out of bounds")
            }
            Self::BufferEmpty => {
                write!(f, "Buffer is empty")
            }
        }
    }
}

pub type SoftResult<T> = std::result::Result<T, SoftError>;
pub type IoResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    GoTo,
    Delete,
}
impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GoTo => write!(f, "{:?}  ", self),
            _ => write!(f, "{:?}", self),
        }
    }
}
impl Mode {
    pub fn color(&self) -> Style {
        match self {
            Self::Normal => Style::default().fg(Color::White).bg(Color::Blue),
            Self::GoTo => Style::default().fg(Color::White).bg(Color::Black),
            Self::Insert => Style::default().fg(Color::White).bg(Color::Green),
            Self::Delete => Style::default().fg(Color::White).bg(Color::Red),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Notification {
    message_type: NotificationType,
    text: Box<str>,
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.message_type {
            NotificationType::Info => "󰬐",
            NotificationType::Error => "",
            NotificationType::Warning => "",
            NotificationType::Success => "",
        };
        write!(f, "{} {}", icon, self.text)
    }
}
impl From<&SoftError> for Notification {
    fn from(err: &SoftError) -> Self {
        let message_type = match err {
            SoftError::OutOfBounds(_) => NotificationType::Warning,
            SoftError::NoLinesLeft => NotificationType::Warning,
            SoftError::KeyUnmapped => NotificationType::Warning,
            SoftError::AlreadyAtTop => NotificationType::Warning,
            SoftError::AlreadyAtBottom => NotificationType::Warning,
            SoftError::NoMoreWordsInLine => NotificationType::Warning,
            SoftError::AlreadyAtLineEnd => NotificationType::Warning,
            SoftError::AlreadyAtLineStart => NotificationType::Warning,
            SoftError::NoCharactersInLine => NotificationType::Warning,
            SoftError::LineDoesNotExist => NotificationType::Error,
            SoftError::CursorOutOfBounds => NotificationType::Warning,
            SoftError::BufferEmpty => NotificationType::Warning,
        };
        Notification {
            message_type,
            text: err.to_string().into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum NotificationType {
    #[default]
    Info,
    Error,
    Warning,
    Success,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    enum MockFile {
        Basic,
        Empty,
        Sparse,
    }

    fn init(file: MockFile) -> App {
        let file_name: String = match file {
            MockFile::Basic => "tests/mocks/basic.txt".into(),
            MockFile::Empty => "tests/mocks/empty.txt".into(),
            MockFile::Sparse => "tests/mocks/sparse.txt".into(),
        };

        let buffer = Buffer::from_file(file_name).unwrap();
        App::new(buffer)
    }

    #[test]
    fn test_move_right() {
        let mut app = init(MockFile::Basic);

        app.move_right();
        app.move_right();
        assert_eq!(app.buffer().cursor().x, 2);
    }

    #[test]
    fn test_move_down() {
        let mut app = init(MockFile::Basic);

        app.move_down().unwrap();
        app.move_down().unwrap();
        assert_eq!(app.buffer().cursor().y, 2);
    }

    #[test]
    fn test_fail_move_down() {
        let mut app = init(MockFile::Sparse);

        app.move_down().unwrap();
        app.move_down().unwrap();
        app.move_down().unwrap();
        assert!(matches!(
            app.move_down(),
            Err(SoftError::OutOfBounds(Direction::Down))
        ));
        assert_eq!(app.buffer().cursor().y, 3);
    }

    #[test]
    fn test_move_up() {
        let mut app = init(MockFile::Basic);

        app.move_down().unwrap();
        app.move_down().unwrap();
        app.move_up().unwrap();
        assert_eq!(app.buffer().cursor().y, 1);
    }

    #[test]
    fn test_fail_move_up() {
        let mut app = init(MockFile::Basic);

        assert!(matches!(
            app.move_up(),
            Err(SoftError::OutOfBounds(Direction::Up))
        ));
        assert_eq!(app.buffer().cursor().y, 0);
    }

    #[test]
    fn test_move_left() {
        let mut app = init(MockFile::Basic);

        app.move_right();
        app.move_left().unwrap();
        assert_eq!(app.buffer().cursor().x, 0);
    }

    #[test]
    fn test_fail_move_left() {
        let mut app = init(MockFile::Basic);

        assert!(matches!(
            app.move_left(),
            Err(SoftError::OutOfBounds(Direction::Left))
        ));
        assert_eq!(app.buffer().cursor().x, 0);
    }

    #[test]
    fn test_move_end_of_line() {
        let mut app = init(MockFile::Basic);

        app.move_end_line().unwrap();
        assert_eq!(app.buffer().cursor().x, 33);
    }

    #[test]
    fn test_fail_move_end_of_line() {
        let mut app = init(MockFile::Sparse);

        app.move_end_line().unwrap();
        assert!(matches!(
            app.move_end_line(),
            Err(SoftError::AlreadyAtLineEnd)
        ));
        assert_eq!(app.buffer().cursor().x, 2);

        app.move_down().unwrap();
        assert!(matches!(
            app.move_end_line(),
            Err(SoftError::NoCharactersInLine)
        ));
        assert_eq!(app.buffer().cursor().x, 0);
    }

    #[test]
    fn test_move_start_of_line() {
        let mut app = init(MockFile::Basic);

        app.move_right();
        app.move_right();
        app.move_start_line().unwrap();
        assert_eq!(app.buffer().cursor().x, 0);
    }

    #[test]
    fn test_fail_move_start_of_line() {
        let mut app = init(MockFile::Basic);

        assert!(matches!(
            app.move_start_line(),
            Err(SoftError::AlreadyAtLineStart)
        ));
        assert_eq!(app.buffer().cursor().x, 0);

        app.move_down().unwrap();
        app.move_down().unwrap();
        app.move_right();
        assert!(matches!(
            app.move_start_line(),
            Err(SoftError::NoCharactersInLine)
        ));
        assert_eq!(app.buffer().cursor().x, 0);
    }

    #[test]
    fn test_move_to_bottom() {
        let mut app = init(MockFile::Basic);

        app.move_to_bottom().unwrap();
        assert_eq!(app.buffer().cursor().y, 3);
    }

    #[test]
    fn test_fail_move_to_bottom() {
        let mut app = init(MockFile::Sparse);

        app.move_down().unwrap();
        app.move_down().unwrap();
        app.move_down().unwrap();
        assert!(matches!(
            app.move_to_bottom(),
            Err(SoftError::AlreadyAtBottom)
        ));
        assert_eq!(app.buffer().cursor().y, 3)
    }

    #[test]
    fn test_move_to_top() {
        let mut app = init(MockFile::Basic);

        app.move_down().unwrap();
        app.move_down().unwrap();
        app.move_to_top().unwrap();
        assert_eq!(app.buffer().cursor().y, 0);
    }

    #[test]
    fn test_fail_move_to_top() {
        let mut app = init(MockFile::Basic);

        assert!(matches!(app.move_to_top(), Err(SoftError::AlreadyAtTop)));
        assert_eq!(app.buffer().cursor().y, 0);
    }
    #[test]
    fn test_move_next_word_start() {
        let mut app = init(MockFile::Basic);

        app.move_next_word_start().unwrap();
        assert_eq!(app.buffer().cursor().x, 2);

        app.move_next_word_start().unwrap();
        assert_eq!(app.buffer().cursor().x, 3);

        app.move_next_word_start().unwrap();
        assert_eq!(app.buffer().cursor().x, 10);

        app.move_left().unwrap();
        app.move_next_word_start().unwrap();
        assert_eq!(app.buffer().cursor().x, 10);
    }
    #[test]
    fn test_fail_move_next_word_start() {
        let mut app = init(MockFile::Sparse);

        app.move_next_word_start().unwrap();
        app.move_next_word_start().unwrap();
        assert!(matches!(
            app.move_next_word_start(),
            Err(SoftError::NoMoreWordsInLine)
        ));
        assert_eq!(app.buffer().cursor().x, 2);

        app.move_right();
        assert!(matches!(
            app.move_next_word_start(),
            Err(SoftError::CursorOutOfBounds)
        ));
        assert_eq!(app.buffer().cursor().x, 3);
    }
    #[test]
    fn test_move_next_word_start_long() {
        let mut app = init(MockFile::Basic);

        app.move_next_word_start_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 10);

        // from whitespace
        app.move_left().unwrap();
        app.move_next_word_start_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 10);

        app.move_next_word_start_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 14);
    }
    #[test]
    fn test_fail_move_next_word_start_long() {
        let mut app = init(MockFile::Sparse);

        assert!(matches!(
            app.move_next_word_start_long(),
            Err(SoftError::NoMoreWordsInLine)
        ));
        assert_eq!(app.buffer().cursor().x, 0);

        app.move_right();
        app.move_right();
        app.move_right();
        app.move_right();
        assert!(matches!(
            app.move_next_word_start_long(),
            Err(SoftError::CursorOutOfBounds)
        ));
        assert_eq!(app.buffer().cursor().x, 4);
    }
    #[test]
    fn test_move_next_word_end() {
        let mut app = init(MockFile::Basic);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 1);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 2);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 8);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 11);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 14);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 16);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 17);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 19);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 24);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 25);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 26);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 27);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 32);

        app.move_next_word_end().unwrap();
        assert_eq!(app.buffer().cursor().x, 33);
    }
    #[test]
    fn test_fail_move_next_word_end() {
        let mut app = init(MockFile::Sparse);

        app.move_next_word_end().unwrap();
        app.move_next_word_end().unwrap();
        assert!(matches!(
            app.move_next_word_end(),
            Err(SoftError::NoMoreWordsInLine)
        ));

        app.move_down().unwrap();
        assert!(matches!(
            app.move_next_word_end(),
            Err(SoftError::CursorOutOfBounds)
        ));
    }
    #[test]
    fn test_move_next_word_end_long() {
        let mut app = init(MockFile::Basic);

        app.move_next_word_end_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 8);

        app.move_next_word_end_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 11);

        app.move_next_word_end_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 14);

        app.move_next_word_end_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 19);

        app.move_next_word_end_long().unwrap();
        assert_eq!(app.buffer().cursor().x, 33);
    }
    #[test]
    fn test_fail_move_next_word_end_long() {
        let mut app = init(MockFile::Sparse);

        app.move_next_word_end_long().unwrap();
        assert!(matches!(
            app.move_next_word_end_long(),
            Err(SoftError::NoMoreWordsInLine)
        ));

        app.move_down().unwrap();
        assert!(matches!(
            app.move_next_word_end_long(),
            Err(SoftError::CursorOutOfBounds)
        ));
    }
    #[test]
    fn test_insert_text() {
        let mut app = init(MockFile::Sparse);

        app.insert_char('t').unwrap();
        app.insert_char('e').unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 'P');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 'e');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 't');

        app.move_right();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 'e');

        app.move_end_line().unwrap();
        app.move_right();
        app.insert_char('s').unwrap();
        app.insert_char('t').unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), '\n');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 't');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 's');

        let mut app = init(MockFile::Empty);
        app.insert_char('s').unwrap();
        app.insert_char('t').unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), '\n');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 't');

        app.move_left().unwrap();
        assert_eq!(app.buffer().char_under_cursor().unwrap(), 's');
    }
}

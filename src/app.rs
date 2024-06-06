use crate::buffer::{self, Buffer, BufferError};
use ratatui::style::{Color, Style};
use std::{
    char,
    error::{self, Error},
    fmt::Display,
};

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
    pub buffer: Buffer,
    mode: Mode,
    notifications: Vec<Notification>,
}

impl App {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            ..Default::default()
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn running_state(&self) -> RunningState {
        self.running_state
    }

    pub fn notifs(&self) -> &[Notification] {
        self.notifications.as_slice()
    }

    pub fn push_notif(&mut self, msg: Notification) {
        self.notifications.push(msg)
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done
    }

    pub fn insert_char_before(&mut self, char: char) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        let on_tail = self.buffer.on_rope_tail(cursor.into());
        if on_tail {
            self.buffer.insert(self.buffer.len(), char.to_string())?;
        } else {
            let char_idx = self.buffer.char_idx_under_pos(cursor.into())?;
            self.buffer.insert(char_idx, char.to_string())?;
        };

        Ok(())
    }

    pub fn delete_lines(&mut self, count: usize) -> AppResult<()> {
        todo!();
        if self.buffer.len_lines() == 0 {
            return Err(AppError::NoMoreLinesToDelete);
        }

        let line_idx = self.buffer.cursor.y;
        let _ = self.buffer.delete_line(line_idx);

        Ok(())
    }

    pub fn move_up(&mut self, count: usize) -> AppResult<()> {
        let lines_remaining = self.buffer.cursor.y;
        self.buffer
            .cursor
            .y
            .checked_sub(count)
            .ok_or(AppError::CantMoveUp(count, lines_remaining))
            .map(|y_pos| {
                self.buffer.cursor.y = y_pos;
            })
    }

    pub fn move_down(&mut self, count: usize) -> AppResult<()> {
        let line_idx_to_move_to = self.buffer.cursor.y + count;
        let line_idx_of_last_line = self.buffer.len_lines() - 1;
        let attempted_line_idx_out_of_bounds =
            line_idx_to_move_to > line_idx_of_last_line;
        if attempted_line_idx_out_of_bounds {
            return Err(AppError::CantMoveDown(
                count,
                line_idx_to_move_to - line_idx_of_last_line,
            ));
        };

        let y_pos = self
            .buffer
            .cursor
            .y
            .checked_add(count)
            .expect("y cursor value overflow");
        self.buffer.cursor.y = y_pos;

        Ok(())
    }

    pub fn move_left(&mut self, count: usize) -> AppResult<()> {
        self.buffer
            .cursor
            .x
            .checked_sub(count)
            .ok_or(AppError::CantMoveLeft(count, self.buffer.cursor.x))
            .map(|x_pos| {
                self.buffer.cursor.x = x_pos;
            })
    }

    pub fn move_right(&mut self, count: usize) {
        let x_pos = self
            .buffer
            .cursor
            .x
            .checked_add(count)
            .expect("x cursor value overflow");

        self.buffer.cursor.x = x_pos;
    }

    /// Moves to the first char in the file. If buffer is empty move to (0,0).
    pub fn move_to_start_of_file(&mut self) -> AppResult<()> {
        self.buffer.cursor.y = 0;
        self.buffer.cursor.x = 0;
        if self.buffer.is_empty() {
            return Err(AppError::NoCharsInFile);
        }

        Ok(())
    }

    /// Moves to the last char in the file. If buffer is empty move to (0,0).
    pub fn move_to_end_of_file(&mut self) -> AppResult<()> {
        match self.buffer.end_pos() {
            Err(err) => match err {
                BufferError::NoCharsInFile => {
                    self.buffer.cursor = (0, 0).into();
                    Err(err.into())
                }
                _ => unreachable!("impossible app state"),
            },
            Ok(v) => {
                self.buffer.cursor = v.into();
                Ok(())
            }
        }
    }

    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    /// Moves to the start of the next word in the line.
    pub fn move_next_word_start(&mut self, count: usize) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        if !self.buffer.in_visual_bounds(cursor.into()) {
            return Err(AppError::CursorOutOfBounds);
        };

        let start = self.buffer.char_idx_line_start(cursor.y)? + cursor.x;
        let end = self.buffer.char_idx_line_end(cursor.y)?;
        let words_in_line = self.buffer.words(start..=end)?;

        let word_idx =
            match self.buffer.char_under_pos(cursor.into())?.is_whitespace() {
                true => count - 1,
                false => count,
            };

        match words_in_line.get(word_idx) {
            None => Err(AppError::NoMoreWordsInLine),
            Some(word) => {
                let first_char =
                    word.first().expect("Chars array for next word is empty");
                self.buffer.cursor.x += first_char.char_idx;

                Ok(())
            }
        }
    }

    pub fn move_next_word_start_long(&mut self, count: usize) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        if !self.buffer.in_visual_bounds(cursor.into()) {
            return Err(AppError::CursorOutOfBounds);
        };

        let start = self.buffer.char_idx_line_start(cursor.y)? + cursor.x;
        let end_of_line = self.buffer.char_idx_line_end(cursor.y)?;
        let words = self.buffer.words_long(start..=end_of_line)?;
        let word_idx =
            match self.buffer.char_under_pos(cursor.into())?.is_whitespace() {
                true => count - 1,
                false => count,
            };

        match words.get(word_idx) {
            None => Err(AppError::NoMoreWordsInLine),
            Some(word) => {
                let last_char =
                    word.first().expect("Chars array for next word is empty");
                self.buffer.cursor.x += last_char.char_idx;

                Ok(())
            }
        }
    }

    pub fn move_next_word_end(&mut self, count: usize) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        if !self.buffer.in_visual_bounds(cursor.into()) {
            return Err(AppError::CursorOutOfBounds);
        };

        let start = self.buffer.char_idx_line_start(cursor.y)? + cursor.x;
        let end = self.buffer.char_idx_line_end(cursor.y)?;
        let words = self.buffer.words(start..=end)?;

        let word_idx = match words.first() {
            Some(word) => {
                if word.len() == 1 {
                    count
                } else {
                    count - 1
                }
            }
            None => return Err(AppError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(AppError::NoMoreWordsInLine),
            Some(word) => {
                let last_char =
                    word.last().expect("Chars array for next word is empty");
                self.buffer.cursor.x += last_char.char_idx;

                Ok(())
            }
        }
    }

    pub fn move_next_word_end_long(&mut self, count: usize) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        if !self.buffer.in_visual_bounds(cursor.into()) {
            return Err(AppError::CursorOutOfBounds);
        };

        let start = self.buffer.char_idx_line_start(cursor.y)? + cursor.x;
        let end = self.buffer.char_idx_line_start(cursor.y + 1)? - 1;
        let words = self.buffer.words_long(start..end)?;

        let word_idx = match words.first() {
            Some(word) => {
                if word.len() == 1 {
                    count
                } else {
                    count - 1
                }
            }
            None => return Err(AppError::CursorOutOfBounds),
        };

        match words.get(word_idx) {
            None => Err(AppError::NoMoreWordsInLine),
            Some(word) => {
                let last_char =
                    word.last().expect("Chars array for next word is empty");
                self.buffer.cursor.x += last_char.char_idx;

                Ok(())
            }
        }
    }

    /// Move to the newline char on line. If it does't exist, error.
    pub fn move_new_line_char(&mut self) -> AppResult<()> {
        todo!()
    }

    /// Move cursor to x=0. Also returns an error if the line has no chars or the cursor is already
    /// x = 0.
    pub fn move_start_line(&mut self) -> AppResult<()> {
        if self.buffer.cursor.x == 0 {
            return Err(AppError::AlreadyAtLineStart);
        };
        let line_idx = self.buffer.cursor.y;
        self.buffer.cursor.x = 0;
        if self.buffer.line(line_idx)?.is_visually_empty() {
            return Err(AppError::LineEmpty);
        }

        Ok(())
    }

    /// Move to the last char of the current + count line .
    /// If there are no chars, then move to x=0.
    pub fn move_end_line(&mut self) -> AppResult<()> {
        let cursor = self.buffer.cursor;
        let on_last_line = self.buffer.cursor.y == self.buffer.len_lines() - 1;
        let curr_line = self.buffer.line(cursor.y)?;

        let char_count = if on_last_line {
            curr_line.len()
        } else {
            curr_line.len() - 1
        };

        if char_count == 0 {
            self.buffer.cursor.x = 0;
            return Err(AppError::LineEmpty);
        };

        let at_line_end = cursor.x == char_count - 1;
        if at_line_end {
            return Err(AppError::AlreadyAtLineEnd);
        }

        self.buffer.cursor.x = char_count - 1;
        Ok(())
    }
}

/// Soft errors that are displayed in the status line.
#[derive(Debug)]
pub enum AppError {
    NoMoreLinesToDelete,
    CantMoveUp(
        usize, // Attempted distance.
        usize, // Actual remaining distance.
    ),
    CantMoveLeft(
        usize, // Attempted distance.
        usize, // Actual remaining distance.
    ),
    CantMoveDown(
        usize, // Attempted distance.
        usize, // Actual remaining distance.
    ),
    AlreadyAtEnd,
    AlreadyAtStart,
    AlreadyAtLineEnd,
    AlreadyAtLineStart,
    CursorOutOfBounds,
    NoCharsInFile,
    NoMoreWordsInLine,
    LineEmpty,
    KeyUnmapped,
    CountRedundant,
}

impl From<BufferError> for AppError {
    // do later
    fn from(err: BufferError) -> Self {
        match err {
            BufferError::NoCharsInFile => AppError::NoCharsInFile,
            BufferError::CursorOutOfBounds => AppError::CursorOutOfBounds,
            _ => AppError::NoMoreLinesToDelete,
        }
    }
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMoreLinesToDelete => write!(f, "No more lines to delete"),
            Self::CantMoveUp(count, remaining) => write!(
                f,
                "Can't move up {} lines, {} lines remaining",
                count, remaining
            ),
            Self::CantMoveLeft(count, remaining) => write!(
                f,
                "Can't move left {} characters, {} characters remaining",
                count, remaining
            ),
            Self::CantMoveDown(count, remaining) => write!(
                f,
                "Can't move down {} lines, {} lines remaining",
                count, remaining
            ),
            Self::AlreadyAtEnd => write!(f, "Already at end"),
            Self::AlreadyAtStart => write!(f, "Already at start"),
            Self::AlreadyAtLineEnd => write!(f, "Already at line end"),
            Self::AlreadyAtLineStart => write!(f, "Already at line start"),
            Self::CursorOutOfBounds => write!(f, "Cursor out of bounds"),
            Self::NoCharsInFile => write!(f, "No end exists, buffer is empty"),
            Self::NoMoreWordsInLine => write!(f, "No more words in line"),
            Self::LineEmpty => write!(f, "No chars in line"),
            Self::KeyUnmapped => write!(f, "Key unmapped"),
            Self::CountRedundant => write!(f, "Count redundant"),
        }
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;

pub type IoResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Default, PartialEq, Copy, Clone)]
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
        write!(f, "{}  {}", icon, self.text)
    }
}

impl From<&AppError> for Notification {
    fn from(err: &AppError) -> Self {
        match err {
            AppError::NoMoreLinesToDelete => Self {
                message_type: NotificationType::Warning,
                text: "no lines to delete".into(),
            },
            AppError::CantMoveUp(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already at top".into(),
            },
            AppError::CantMoveDown(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already at bottom".into(),
            },
            AppError::CantMoveLeft(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already leftmost".into(),
            },
            AppError::AlreadyAtStart => Self {
                message_type: NotificationType::Warning,
                text: "already at file start".into(),
            },
            AppError::AlreadyAtEnd => Self {
                message_type: NotificationType::Warning,
                text: "already at file end".into(),
            },
            AppError::AlreadyAtLineStart => Self {
                message_type: NotificationType::Warning,
                text: "already at line start".into(),
            },
            AppError::AlreadyAtLineEnd => Self {
                message_type: NotificationType::Warning,
                text: "already at line end".into(),
            },
            AppError::CursorOutOfBounds => Self {
                message_type: NotificationType::Error,
                text: "cursor out of bounds".into(),
            },
            AppError::NoMoreWordsInLine => Self {
                message_type: NotificationType::Warning,
                text: "no more words in line".into(),
            },
            AppError::LineEmpty => Self {
                message_type: NotificationType::Warning,
                text: "no chars in line".into(),
            },
            AppError::NoCharsInFile => Self {
                message_type: NotificationType::Warning,
                text: "no chars in file".into(),
            },
            AppError::KeyUnmapped => Self {
                message_type: NotificationType::Warning,
                text: "key unmapped".into(),
            },
            AppError::CountRedundant => Self {
                message_type: NotificationType::Warning,
                text: "count redundant".into(),
            },
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

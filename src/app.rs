use crate::buffer::{self, Buffer};
use ratatui::style::{Color, Style};
use std::{char, fmt::Display};

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

    pub fn insert_char_before(&mut self, char: char) -> Result<()> {
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

    pub fn delete_lines(&mut self, count: usize) -> Result<()> {
        if count == 0 {
            return Ok(());
        }

        let line_idx = self.buffer.cursor.y;
        let in_bound = line_idx + count <= self.buffer.len_lines();
        if !in_bound || self.buffer.is_empty() {
            return Err(Error::NoMoreLinesToDelete);
        };

        let end_line_idx = line_idx + count - 1;
        let len_lines = self.buffer.len_lines();
        let end_line_on_last_line = end_line_idx == len_lines - 1;
        let start_on_first_line = line_idx == 0;
        let start_idx = if end_line_on_last_line && !start_on_first_line {
            self.move_up(1)?;
            self.buffer.char_idx_line_end(line_idx - 1)?
        } else if start_on_first_line {
            0
        } else {
            // !end_line_on_last_line && !start_on_first_line
            self.move_up(1)?;
            self.buffer.char_idx_line_start(line_idx - 1)?
        };

        let end_idx = self.buffer.char_idx_line_end(end_line_idx)?;
        self.buffer.remove(start_idx..=end_idx)?;

        Ok(())
    }

    pub fn move_up(&mut self, count: usize) -> Result<()> {
        let lines_remaining = self.buffer.cursor.y;
        self.buffer
            .cursor
            .y
            .checked_sub(count)
            .ok_or(Error::CantMoveUp(count, lines_remaining))
            .map(|y_pos| {
                self.buffer.cursor.y = y_pos;
            })
    }

    pub fn move_down(&mut self, count: usize) -> Result<()> {
        let line_idx_to_move_to = self.buffer.cursor.y + count;
        let line_idx_of_last_line = self.buffer.len_lines() - 1;
        let attempted_line_idx_out_of_bounds =
            line_idx_to_move_to > line_idx_of_last_line;
        if attempted_line_idx_out_of_bounds {
            return Err(Error::CantMoveDown(
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

    pub fn move_left(&mut self, count: usize) -> Result<()> {
        self.buffer
            .cursor
            .x
            .checked_sub(count)
            .ok_or(Error::CantMoveLeft(count, self.buffer.cursor.x))
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
    pub fn move_to_start_of_file(&mut self) -> Result<()> {
        self.buffer.cursor.y = 0;
        self.buffer.cursor.x = 0;
        if self.buffer.is_empty() {
            return Err(Error::NoCharsInFile);
        }

        Ok(())
    }

    /// Moves to the last char in the file. If buffer is empty move to (0,0).
    pub fn move_to_end_of_file(&mut self) -> Result<()> {
        match self.buffer.end_pos() {
            Err(err) => match err {
                buffer::Error::NoCharsInFile => {
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

    fn ensure_cursor_in_visual_bounds(&self) -> Result<()> {
        let cursor = self.buffer.cursor;
        if !self.buffer.in_visual_bounds(cursor.into()) {
            return Err(Error::CursorOutOfBounds);
        }

        Ok(())
    }

    // prev long end
    fn word_distance(
        &mut self,
        count: usize,
        direction: Direction,
        length: Length,
        boundary: Boundary,
    ) -> Result<usize> {
        let cursor = self.buffer.cursor;
        let start = self.buffer.char_idx_under_pos(cursor.into())?;

        match (direction, length, boundary) {
            (Direction::Forward, Length::Short, Boundary::Start) => {
                let end = self.buffer.char_idx_line_end(cursor.y)?;

                let is_whitespace =
                    self.buffer.char_under_pos(cursor.into())?.is_whitespace();
                let word_idx = if is_whitespace { count - 1 } else { count };

                self.buffer
                    .words(start..=end)?
                    .get(word_idx)
                    .map(|word| word.first().unwrap().distance)
                    .ok_or(Error::NoMoreWordsInLine)
            }
            (Direction::Forward, Length::Short, Boundary::End) => {
                self.ensure_cursor_in_visual_bounds()?;

                let end = self.buffer.char_idx_line_end(cursor.y)?;
                let words = self.buffer.words(start..=end)?;
                let word_idx = words
                    .first()
                    .map(|word| if word.len() == 1 { count } else { count - 1 })
                    .ok_or(Error::CursorOutOfBounds)?;

                words
                    .get(word_idx)
                    .map(|word| word.last().unwrap().distance)
                    .ok_or(Error::NoMoreWordsInLine)
            }
            (Direction::Forward, Length::Long, Boundary::Start) => {
                let end = self.buffer.char_idx_line_end(cursor.y)?;
                let is_whitespace =
                    self.buffer.char_under_pos(cursor.into())?.is_whitespace();
                let word_idx = if is_whitespace { count - 1 } else { count };

                self.buffer
                    .words_long(start..=end)?
                    .get(word_idx)
                    .map(|word| word.first().unwrap().distance)
                    .ok_or(Error::NoMoreWordsInLine)
            }
            (Direction::Forward, Length::Long, Boundary::End) => {
                self.ensure_cursor_in_visual_bounds()?;

                let end = self.buffer.char_idx_line_end(cursor.y)?;
                let words = self.buffer.words_long(start..=end)?;
                let word_idx = words
                    .first()
                    .map(|word| if word.len() == 1 { count } else { count - 1 })
                    .ok_or(Error::CursorOutOfBounds)?;

                words
                    .get(word_idx)
                    .map(|word| word.last().unwrap().distance)
                    .ok_or(Error::NoMoreWordsInLine)
            }
            (Direction::Backward, Length::Short, Boundary::Start) => {
                if cursor.x == 0 {
                    return Err(Error::NoMoreWordsInLine);
                }

                // if out of bounds {
                // if line_empty() {
                // retrun 0
                // }
                // return END_OF_LINE - 1
                // }

                let on_empty_last_line = cursor.y
                    == self.buffer.len_lines() - 1
                    && self.buffer.line(cursor.y)?.chars().len() == 0;
                if on_empty_last_line {
                    return Err(Error::NoMoreWordsInLine);
                }

                // Line is not empty.
                let start = self.buffer.char_idx_line_start(cursor.y)?;
                let end = std::cmp::min(
                    self.buffer.char_idx_line_start(cursor.y)? + cursor.x - 1, // the char before
                    self.buffer.char_idx_line_end(cursor.y)?,
                );

                let mut words_in_line = self.buffer.words(start..=end)?;
                words_in_line.reverse();

                let char_idx_from_start = words_in_line
                    .get(count - 1)
                    .map(|word| word.first().unwrap().distance)
                    .ok_or(Error::NoMoreWordsInLine);

                Ok(cursor.x - char_idx_from_start?)
            }
            (Direction::Backward, Length::Short, Boundary::End) => {
                todo!()
            }
            (Direction::Backward, Length::Long, Boundary::Start) => {
                todo!()
            }
            (Direction::Backward, Length::Long, Boundary::End) => {
                todo!()
            }
        }
    }

    /// Moves to the start of the next word in the line.
    pub fn move_word(
        &mut self,
        count: usize,
        direction: Direction,
        length: Length,
        boundary: Boundary,
    ) -> Result<()> {
        match direction {
            Direction::Forward => {
                self.buffer.cursor.x +=
                    self.word_distance(count, direction, length, boundary)?
            }
            Direction::Backward => {
                let distance =
                    self.word_distance(count, direction, length, boundary)?;
                if self.buffer.cursor.x < distance {
                    return Err(Error::NoMoreWordsInLine);
                }
                self.buffer.cursor.x -= distance
            }
        };

        Ok(())
    }

    pub fn delete_word(
        &mut self,
        count: usize,
        direction: Direction,
        length: Length,
        boundary: Boundary,
    ) -> Result<()> {
        match direction {
            Direction::Forward => {
                let start_char_idx = self
                    .buffer
                    .char_idx_under_pos(self.buffer.cursor.into())?;
                let end_char_idx = self
                    .word_distance(count, direction, length, boundary)?
                    + start_char_idx;

                self.buffer.remove(start_char_idx..=end_char_idx)?;
            }
            Direction::Backward => {
                let end_char_idx = self
                    .buffer
                    .char_idx_under_pos(self.buffer.cursor.into())?;
                let start_char_idx = end_char_idx
                    - self.word_distance(count, direction, length, boundary)?;

                self.buffer.remove(start_char_idx..=end_char_idx)?;
            }
        }

        Ok(())
    }

    /// Move to the newline char on line. If it does't exist, error.
    pub fn move_new_line_char(&mut self) -> Result<()> {
        todo!()
    }

    /// Move cursor to x=0. Also returns an error if the line has no chars or the cursor is already
    /// x = 0.
    pub fn move_start_line(&mut self) -> Result<()> {
        let line_idx = self.buffer.cursor.y;
        let at_line_start = self.buffer.cursor.x == 0;
        self.buffer.cursor.x = 0;

        if self.buffer.line(line_idx)?.is_visually_empty() {
            return Err(Error::LineEmpty);
        }

        if at_line_start {
            return Err(Error::AlreadyAtLineStart);
        };

        Ok(())
    }

    /// Move to the last char of the current + count line .
    /// If there are no chars, then move to x=0.
    pub fn move_end_line(&mut self) -> Result<()> {
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
            return Err(Error::LineEmpty);
        };

        let at_line_end = cursor.x == char_count - 1;
        if at_line_end {
            return Err(Error::AlreadyAtLineEnd);
        }

        self.buffer.cursor.x = char_count - 1;
        Ok(())
    }
}

/// Soft errors that are displayed in the status line.
#[derive(Debug)]
pub enum Error {
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

impl From<buffer::Error> for Error {
    // do later
    fn from(err: buffer::Error) -> Self {
        match err {
            buffer::Error::NoCharsInFile => Error::NoCharsInFile,
            buffer::Error::CursorOutOfBounds => Error::CursorOutOfBounds,
            _ => Error::NoMoreLinesToDelete,
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
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

pub type Result<T> = std::result::Result<T, Error>;

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

impl From<&Error> for Notification {
    fn from(err: &Error) -> Self {
        match err {
            Error::NoMoreLinesToDelete => Self {
                message_type: NotificationType::Warning,
                text: "no lines to delete".into(),
            },
            Error::CantMoveUp(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already at top".into(),
            },
            Error::CantMoveDown(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already at bottom".into(),
            },
            Error::CantMoveLeft(count, remaining) => Self {
                message_type: NotificationType::Warning,
                text: "already leftmost".into(),
            },
            Error::AlreadyAtStart => Self {
                message_type: NotificationType::Warning,
                text: "already at file start".into(),
            },
            Error::AlreadyAtEnd => Self {
                message_type: NotificationType::Warning,
                text: "already at file end".into(),
            },
            Error::AlreadyAtLineStart => Self {
                message_type: NotificationType::Warning,
                text: "already at line start".into(),
            },
            Error::AlreadyAtLineEnd => Self {
                message_type: NotificationType::Warning,
                text: "already at line end".into(),
            },
            Error::CursorOutOfBounds => Self {
                message_type: NotificationType::Error,
                text: "cursor out of bounds".into(),
            },
            Error::NoMoreWordsInLine => Self {
                message_type: NotificationType::Warning,
                text: "no more words in line".into(),
            },
            Error::LineEmpty => Self {
                message_type: NotificationType::Warning,
                text: "no chars in line".into(),
            },
            Error::NoCharsInFile => Self {
                message_type: NotificationType::Warning,
                text: "no chars in file".into(),
            },
            Error::KeyUnmapped => Self {
                message_type: NotificationType::Warning,
                text: "key unmapped".into(),
            },
            Error::CountRedundant => Self {
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

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
}

pub enum Boundary {
    Start,
    End,
}

pub enum Length {
    Long,
    Short,
}

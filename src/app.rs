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
    cursor: Cursor,
    messages: Vec<Message>,
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
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }
    pub fn running_state_mut(&mut self) -> &mut RunningState {
        &mut self.running_state
    }
    pub fn messages(&self) -> &[Message] {
        self.messages.as_slice()
    }
    pub fn push_msg(&mut self, msg: Message) {
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
    pub fn scroll_pos(&self) -> usize {
        self.cursor.y
    }
    pub fn insert_char(&mut self, char: char) -> AppResult {
        let byte_idx = self
            .buffer()
            .byte_idx_of_cursor_pos(self.cursor())
            .expect("out of bounds");
        self.buffer.insert(byte_idx, char.to_string());
        self.move_right()?;
        Ok(())
    }
    pub fn delete_line(&mut self) -> AppResult {
        if self.scroll_pos() == 0 {
            return Err(AppError::NoLinesLeft);
        }
        let line_idx = self.scroll_pos();
        self.buffer.remove_line(line_idx);
        Ok(())
    }
    pub fn move_up(&mut self) -> AppResult {
        self.cursor
            .y
            .checked_sub(1)
            .ok_or(AppError::OutOfBounds(Direction::Down))
            .map(|y_pos| {
                self.cursor.y = y_pos;
            })
    }
    pub fn move_down(&mut self) -> AppResult {
        let cursor_on_last_line = self.buffer.len_lines() == self.scroll_pos() + 1;
        if cursor_on_last_line {
            return Err(AppError::OutOfBounds(Direction::Down));
        };
        let y_pos = self
            .cursor
            .y
            .checked_add(1)
            .expect("y cursor value overflow");
        self.cursor.y = y_pos;
        Ok(())
    }
    pub fn move_left(&mut self) -> AppResult {
        self.cursor
            .x
            .checked_sub(1)
            .ok_or(AppError::OutOfBounds(Direction::Left))
            .map(|x_pos| {
                self.cursor.x = x_pos;
            })
    }
    pub fn move_right(&mut self) -> AppResult {
        self.cursor
            .x
            .checked_add(1)
            .ok_or(AppError::OutOfBounds(Direction::Right))
            .map(|x_pos| {
                self.cursor.x = x_pos;
            })
    }
    pub fn move_to_top(&mut self) -> AppResult {
        if self.cursor.y == 0 {
            return Err(AppError::AlreadyAtTop);
        }
        self.cursor.y = 0;
        Ok(())
    }
    pub fn move_to_bottom(&mut self) -> AppResult {
        if self.cursor.y == 0 {
            return Err(AppError::AlreadyAtBottom);
        }
        self.cursor.y = self.buffer.len_lines() - 1;
        Ok(())
    }
    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
    pub fn move_next_word_start(&mut self) -> AppResult {
        // let line_idx = self.scroll_pos();
        // let line = self.buffer().line(line_idx);
        // get line char idx of start of curr line
        // get char idx of pos
        // check if there is a next start of word
        // if not, return err
        // if so, then move to that char idx
        todo!()
    }
    pub fn move_start_line(&mut self) -> AppResult {
        let left_bound = self.buffer().line_numb_col_width();
        if self.cursor.x == left_bound {
            return Err(AppError::NoMoreWordsInLine);
        }
        self.cursor.x = left_bound;
        Ok(())
    }
    pub fn move_end_line(&mut self) -> AppResult {
        let line_idx = self.scroll_pos();
        let line = self.buffer().line(line_idx);
        let chars_count = line.chars().count();
        let left_bound = self.buffer().line_numb_col_width();
        if chars_count == 0 {
            if self.cursor.x == chars_count {
                return Err(AppError::AlreadyAtLineEnd);
            };
            self.cursor.x = left_bound;
        } else {
            if self.cursor.x == left_bound + chars_count - 1 {
                return Err(AppError::AlreadyAtLineEnd);
            };
            self.cursor.x = left_bound + chars_count - 1
        }
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
pub enum AppError {
    OutOfBounds(Direction),
    NoLinesLeft,
    KeyUnmapped,
    AlreadyAtTop,
    AlreadyAtBottom,
    NoMoreWordsInLine,
    AlreadyAtLineStart,
    AlreadyAtLineEnd,
}
impl Error for AppError {}
impl Display for AppError {
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
        }
    }
}

pub type AppResult = std::result::Result<(), AppError>;
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

#[derive(Debug, Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Message {
    message_type: MessageType,
    text: Box<str>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.message_type {
            MessageType::Info => "󰬐",
            MessageType::Error => "",
            MessageType::Warning => "",
            MessageType::Success => "",
        };
        write!(f, "{} {}", icon, self.text)
    }
}
impl From<&AppError> for Message {
    fn from(err: &AppError) -> Self {
        let message_type = match err {
            AppError::OutOfBounds(_) => MessageType::Warning,
            AppError::NoLinesLeft => MessageType::Warning,
            AppError::KeyUnmapped => MessageType::Warning,
            AppError::AlreadyAtTop => MessageType::Warning,
            AppError::AlreadyAtBottom => MessageType::Warning,
            AppError::NoMoreWordsInLine => MessageType::Warning,
            AppError::AlreadyAtLineEnd => MessageType::Warning,
            AppError::AlreadyAtLineStart => MessageType::Warning,
        };
        Message {
            message_type,
            text: err.to_string().into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum MessageType {
    #[default]
    Info,
    Error,
    Warning,
    Success,
}

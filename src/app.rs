use std::{
    char,
    error::{self},
    fmt::Display,
};

use ratatui::style::{Color, Style};

use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

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
            Self::Normal => Style::default().fg(Color::White).bg(Color::DarkGray),
            Self::GoTo => Style::default().fg(Color::White).bg(Color::DarkGray),
            Self::Insert => Style::default().fg(Color::White).bg(Color::DarkGray),
            Self::Delete => Style::default().fg(Color::White).bg(Color::DarkGray),
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
pub struct App {
    pub running_state: RunningState,
    pub buffer: Buffer,
    pub logger: Logger,
    pub mode: Mode,
    pub cursor: (usize, usize),
}

impl App {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            ..Default::default()
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done;
    }

    pub fn scroll_pos(&self) -> usize {
        self.cursor.1
    }

    pub fn insert_char(&mut self, char: char) {
        let byte_idx = self
            .buffer
            .byte_idx_of_cursor_pos(self.cursor)
            .expect("out of bounds");
        self.buffer.insert(byte_idx, char.to_string());
        self.move_right();
    }

    pub fn delete_line(&mut self) {
        let line_idx = self.scroll_pos();
        self.buffer.remove_line(line_idx);
    }

    pub fn move_up(&mut self) {
        if let Some(res) = self.cursor.1.checked_sub(1) {
            self.cursor.1 = res;
        }
    }

    pub fn move_down(&mut self) {
        if self.buffer.len_lines() != self.scroll_pos() + 1 {
            // prevent scrolling over
            if let Some(res) = self.cursor.1.checked_add(1) {
                self.cursor.1 = res;
            }
        }
    }

    pub fn move_left(&mut self) {
        if let Some(res) = self.cursor.0.checked_sub(1) {
            self.cursor.0 = res;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(res) = self.cursor.0.checked_add(1) {
            self.cursor.0 = res;
        }
    }

    pub fn move_to_top(&mut self) {
        self.cursor.1 = 0;
    }

    pub fn move_to_bottom(&mut self) {
        self.cursor.1 = self.buffer.len_lines() - 1;
    }

    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    pub fn move_next_word_start(&mut self) {
        // get line char idx of start of curr line
        // get char idx of pos
        // check if there is a next start of word
        // if not, return err
        // if so, then move to that char idx
        todo!()
    }

    pub fn move_start_line(&mut self) {
        self.cursor.0 = 0;
    }

    pub fn move_end_line(&mut self) {
        todo!()
    }
}

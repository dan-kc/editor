use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};
use ratatui::style::{Color, Style};
use std::{
    char,
    error::{self},
    fmt::Display,
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

#[derive(Debug, Default)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

impl RunningState {}

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
    buffer: Buffer,
    logger: Logger,
    mode: Mode,
    cursor: Cursor,
}

impl App {
    pub fn new(buffer: buffer::Buffer) -> Self {
        // TODO: Get cursor defaults
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
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }
    fn set_running_state(&mut self, state: RunningState) {
        self.running_state = state
    }
    #[allow(dead_code)]
    fn app_state(&self) -> &RunningState {
        &self.running_state
    }
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    pub fn quit(&mut self) {
        self.set_running_state(RunningState::Done);
    }
    pub fn scroll_pos(&self) -> usize {
        self.cursor.y()
    }
    pub fn insert_char(&mut self, char: char) {
        let byte_idx = self
            .buffer()
            .byte_idx_of_cursor_pos(self.cursor())
            .expect("out of bounds");
        self.buffer.insert(byte_idx, char.to_string());
        self.move_right();
    }
    pub fn delete_line(&mut self) {
        let line_idx = self.scroll_pos();
        self.buffer.remove_line(line_idx);
    }
    pub fn move_up(&mut self) {
        if let Some(res) = self.cursor.y().checked_sub(1) {
            self.cursor.set_y(res);
        }
    }
    pub fn move_down(&mut self) {
        if self.buffer.len_lines() != self.scroll_pos() + 1 {
            if let Some(res) = self.cursor.y().checked_add(1) {
                self.cursor.set_y(res);
            }
        }
    }
    pub fn move_left(&mut self) {
        if let Some(res) = self.cursor.x().checked_sub(1) {
            self.cursor.set_x(res);
        }
    }
    pub fn move_right(&mut self) {
        if let Some(res) = self.cursor.x().checked_add(1) {
            self.cursor.set_x(res);
        }
    }
    pub fn move_to_top(&mut self) {
        self.cursor.set_y(0);
    }
    pub fn move_to_bottom(&mut self) {
        self.cursor.set_y(self.buffer.len_lines() - 1);
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
        self.cursor.set_x(0);
    }
    pub fn move_end_line(&mut self) {
        todo!()
    }
}

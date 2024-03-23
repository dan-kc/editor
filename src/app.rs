use std::error;

use crate::buffer::{self, Buffer};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    pub fn get_text(self) -> String {
        match self {
            Mode::Normal => String::from("Normal"),
            Mode::Insert => String::from("Insert"),
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub buffer: Buffer,
    pub mode: Mode,
    pub cursor: (u8, u8),
}

impl Default for App {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            mode: Mode::Normal,
            running: true,
            cursor: (0, 0),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            mode: Mode::Normal,
            running: true,
            cursor: (0, 0),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_scroll_pos(&self) -> &u8 {
        &self.cursor.1
    }

    pub fn move_up(&mut self) {
        if let Some(res) = self.cursor.1.checked_sub(1) {
            self.cursor.1 = res;
        }
    }

    pub fn move_down(&mut self) {
        if self.buffer.text.len_lines() as u8 != *self.get_scroll_pos() + 1 {
            // prevent scrolling over
            if let Some(res) = self.cursor.1.checked_add(1) {
                self.cursor.1 = res;
            }
        }
    }

}

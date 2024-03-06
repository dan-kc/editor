use std::error;

use crate::buffer::{self, Buffer};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub buffer: Buffer,
    pub counter: u8,
    cursor: (u8, u8),
}

impl Default for App {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            running: true,
            counter: 0,
            cursor: (0,0),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            running: true,
            counter: 0,
            cursor: (0,0),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

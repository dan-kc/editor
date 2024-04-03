use std::{
    char,
    error::{self},
};

use crate::{
    buffer::{self, Buffer},
    logger::Logger,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// pub type LoggedError {
// }

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    GoTo,
    Delete,
}

impl Mode {
    pub fn get_text(self) -> String {
        match self {
            Mode::Normal => String::from("Normal"),
            Mode::Insert => String::from("Insert"),
            Mode::GoTo => String::from("Go To "),
            Mode::Delete => String::from("Delete"),
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub buffer: Buffer,
    pub logger: Logger,
    pub mode: Mode,
    pub cursor: (u8, u8),
}

impl Default for App {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            mode: Mode::Normal,
            logger: Logger::new(),
            running: true,
            cursor: (0, 0),
        }
    }
}

impl App {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            mode: Mode::Normal,
            logger: Logger::new(),
            running: true,
            cursor: (0, 0),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn scroll_pos(&self) -> &u8 {
        &self.cursor.1
    }

    pub fn insert(&mut self, char: char) {
        let line_idx = *self.scroll_pos() as usize;
        let char_idx = self.buffer.rope.line_to_char(line_idx) + self.cursor.0 as usize;
        self.buffer.rope.insert(char_idx, &char.to_string());
        self.move_right();
    }

    fn last_line(&mut self) -> bool {
        self.cursor.1 as usize == self.buffer.rope.len_lines()
    }

    pub fn delete_line(&mut self) {
        let line_idx = *self.scroll_pos() as usize;
        let start = self.buffer.rope.line_to_char(line_idx);
        let end = start + self.curr_len_line();
        self.buffer.rope.try_remove(start..=end).expect("pain");
    }

    pub fn move_up(&mut self) {
        if let Some(res) = self.cursor.1.checked_sub(1) {
            self.cursor.1 = res;
        }
    }

    pub fn move_down(&mut self) {
        if self.buffer.rope.len_lines() as u8 != *self.scroll_pos() + 1 {
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
        self.cursor.1 = self.buffer.rope.len_lines() as u8 - 1;
    }

    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    // fn char_idx(&mut self, line_idx: usize) -> AppResult<usize> {
    //     if self.in_bounds(line_idx) {
    //         let error = Err("out of bounds".to_string());
    //     }
    //     return Ok(self.buffer.rope.line_to_char(line_idx) + self.cursor.0 as usize);
    // }

    fn curr_len_line(&mut self) -> usize {
        let line_idx = *self.scroll_pos() as usize;
        self.len_line(line_idx)
    }

    fn len_line(&mut self, line_idx: usize) -> usize {
        let start = self.buffer.rope.line_to_char(line_idx);
        let end = self.buffer.rope.line_to_char(line_idx + 1) - 1;
        end - start
    }

    fn in_bounds(&mut self, line_idx: usize) -> bool {
        self.cursor.0 as usize <= self.len_line(line_idx)
    }

    pub fn move_next_word_start(&mut self) {
        // get line char idx of start of curr line
        // get char idx of pos
        // check if there is a next start of word
        // if not, return err
        // if so, then move to that char idx
    }
}

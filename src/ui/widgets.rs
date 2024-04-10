use crate::{
    app::{App, Cursor, Mode},
    buffer::Buffer,
    logger::Logger,
};
use ratatui::prelude::*;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};
use std::{ops::RangeBounds, usize};

#[allow(dead_code)]
pub struct GitSummary<'a> {
    app: &'a App,
}
impl<'a> GitSummary<'a> {
    pub fn new(app: &'a App) -> Self {
        GitSummary { app }
    }
}

impl<'a> Widget for GitSummary<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        for i in 0..area.height {
            let ratatui_line = Line::raw("Git info placeholder");
            _ = buf.set_line(area.x, area.y + i, &ratatui_line, area.width)
        }
    }
}

pub struct UpperTextArea<'a> {
    buffer: &'a Buffer,
    scroll_pos: usize,
}

impl<'a> UpperTextArea<'a> {
    pub fn new(buffer: &'a Buffer, scroll_pos: usize) -> Self {
        UpperTextArea { buffer, scroll_pos }
    }
}

impl<'a> Widget for UpperTextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // render line cols
        for i in 0..area.height {
            let rope_idx = self.scroll_pos - area.height as usize + i as usize;
            let line_numb = self.buffer.numb_col(rope_idx);
            let ratatui_line = Span::raw(line_numb);
            _ = buf.set_span(area.x, area.y + i, &ratatui_line, area.width)
        }

        for i in 0..area.height {
            let rope_idx = self.scroll_pos - area.height as usize + i as usize;
            let line = self.buffer.line(rope_idx).populate_fill_chars(); // panics
            let ratatui_line = Span::raw(line);
            _ = buf.set_span(
                self.buffer.line_numb_col_width() as u16 + area.x,
                area.y + i,
                &ratatui_line,
                area.width,
            )
        }
    }
}

pub struct Logs<'a> {
    logger: &'a Logger,
}

impl<'a> Logs<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        Logs { logger }
    }
}

impl<'a> Widget for Logs<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        for i in 0..area.height {
            let ratatui_line = match self.logger.logs().iter().rev().nth(i as usize) {
                Some(log) => Line::from(log.to_string()),
                None => Line::raw("PLACEHOLDER"),
            };
            _ = buf.set_line(area.x, area.y + i, &ratatui_line, area.width)
        }
    }
}

pub struct LowerTextArea<'a> {
    buffer: &'a Buffer,
    scroll_pos: usize,
}

impl<'a> LowerTextArea<'a> {
    pub fn new(buffer: &'a Buffer, scroll_pos: usize) -> Self {
        LowerTextArea { buffer, scroll_pos }
    }
}

impl<'a> Widget for LowerTextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // render line cols
        for i in 0..area.height {
            let rope_idx = self.scroll_pos + 1 + i as usize;
            let line_numb = self.buffer.numb_col(rope_idx);
            let ratatui_line = Span::raw(line_numb);
            _ = buf.set_span(area.x, area.y + i, &ratatui_line, area.width)
        }
        for i in 0..area.height {
            let rope_idx = self.scroll_pos + 1 + i as usize;
            let line = self.buffer.line(rope_idx).to_string().populate_fill_chars(); // panics
            let ratatui_line = Span::raw(line);
            _ = buf.set_span(self.buffer.line_numb_col_width() as u16 + area.x, area.y + i, &ratatui_line, area.width)
        }
    }
}

pub struct StatusLine<'a> {
    mode: &'a Mode,
    buffer: &'a Buffer,
}

impl<'a> StatusLine<'a> {
    pub fn new(mode: &'a Mode, buffer: &'a Buffer) -> Self {
        StatusLine { mode, buffer }
    }
}

impl<'a> Widget for StatusLine<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut text = self.mode.to_string();
        text.push(' ');
        text.insert(0, ' ');
        let style = self.mode.color();
        let ratatui_line = Span::styled(text, style);
        _ = buf.set_span(self.buffer.line_numb_col_width() as u16, area.y, &ratatui_line, area.width);
    }
}

#[allow(dead_code)]
pub struct CursorLine<'a> {
    buffer: &'a Buffer,
    cursor: &'a Cursor,
    scroll_pos: usize,
    mode: &'a Mode,
}

impl<'a> CursorLine<'a> {
    pub fn new(buffer: &'a Buffer, cursor: &'a Cursor, scroll_pos: usize, mode: &'a Mode) -> Self {
        CursorLine {
            scroll_pos,
            buffer,
            cursor,
            mode,
        }
    }
}

impl<'a> Widget for CursorLine<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let scroll_pos = self.scroll_pos;
        // render line cols
        let line_numb = self.buffer.numb_col(scroll_pos);
        let ratatui_line = Span::raw(line_numb);
        _ = buf.set_span(area.x, area.y, &ratatui_line, area.width);

        let style = Style::default().fg(Color::White).bg(Color::Black);
        let cursor_style = self.mode.color();
        #[rustfmt::skip]
        let line = self.buffer.line(scroll_pos).to_string().populate_fill_chars(); // panics
        let ratatui_line = Span::styled(line.to_string(), style);
        let ratatui_cursor = Span::styled(" ", cursor_style);
        buf.set_style(area, style);
        _ = buf.set_span(area.x + self.buffer.line_numb_col_width() as u16, area.y, &ratatui_line, area.width);
        _ = buf.set_span(
            area.x + self.cursor.x() as u16,
            area.y,
            &ratatui_cursor,
            area.width,
        );
    }
}

pub trait PopulateFillChars {
    fn populate_fill_chars(&mut self) -> Self;
}

impl PopulateFillChars for String {
    fn populate_fill_chars(&mut self) -> Self {
        self.replace(' ', "·")
    }
}

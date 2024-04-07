use crop::Rope;
use ratatui::prelude::*;
use std::usize;

use crate::{app::Mode, logger::Logger};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

pub struct UpperTextArea<'a> {
    rope: &'a Rope,
    scroll_pos: &'a usize,
}

impl<'a> UpperTextArea<'a> {
    pub fn new(app: &'a crate::app::App) -> Self {
        UpperTextArea {
            rope: &app.buffer.rope,
            scroll_pos: app.scroll_pos(),
        }
    }
}

impl<'a> Widget for UpperTextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let scroll_pos = *self.scroll_pos as u16;
        let top_info_line_count = if area.height >= scroll_pos {
            area.height - *self.scroll_pos as u16
        } else {
            0
        };
        for i in 0..top_info_line_count {
            let ratatui_line = Line::raw("----UPPER INFO----");
            _ = buf.set_line(0, i, &ratatui_line, area.width)
        }

        let text_line_count = std::cmp::min(scroll_pos, area.height);
        for i in 0..text_line_count {
            let line_number = if scroll_pos >= area.height {
                (i + scroll_pos - area.height) as usize
            } else {
                i as usize
            };
            #[rustfmt::skip]
            let line = self.rope.line(line_number).to_string().populate_fill_chars(); // panics
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(0, i + top_info_line_count, &ratatui_line, area.width)
        }
    }
}

pub struct LowerTextArea<'a> {
    rope: &'a Rope,
    scroll_pos: &'a usize,
    logger: &'a Logger,
}

impl<'a> LowerTextArea<'a> {
    pub fn new(app: &'a crate::app::App) -> Self {
        LowerTextArea {
            logger: &app.logger,
            rope: &app.buffer.rope,
            scroll_pos: app.scroll_pos(),
        }
    }
}

impl<'a> Widget for LowerTextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let scroll_pos = *self.scroll_pos as u16;
        let text_start = scroll_pos as usize + 1;
        let text_end = std::cmp::min(text_start + area.height as usize, self.rope.line_len());
        for i in text_start..text_end {
            #[rustfmt::skip]
            let line = self.rope.line(i).to_string().populate_fill_chars(); // panics
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(
                area.x,
                area.y + (i - text_start) as u16,
                &ratatui_line,
                area.width,
            )
        }

        let info_line_count = area.height as usize - (text_end - text_start);
        for i in 0..info_line_count {
            let ratatui_line = match self.logger.logs.iter().rev().nth(i) {
                Some(log) => {
                    let log_string = format!("{}", log);
                    Line::from(log_string)
                }
                None => Line::raw("------LOWER INFO PLACEHOLDER------"),
            };
            _ = buf.set_line(
                area.x,
                area.y + (i + (text_end - text_start)) as u16,
                &ratatui_line,
                area.width,
            )
        }
    }
}

pub struct StatusLine<'a> {
    mode: &'a Mode,
}

impl<'a> StatusLine<'a> {
    pub fn new(mode: &'a Mode) -> Self {
        StatusLine { mode }
    }
}

impl<'a> Widget for StatusLine<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut text = self.mode.to_string();
        text.push(' ');
        text.insert(0, ' ');
        let style = self.mode.color();
        let ratatui_line = Span::styled(text, style);
        _ = buf.set_span(area.x, area.y, &ratatui_line, area.width);
    }
}

#[allow(dead_code)]
pub struct CursorLine<'a> {
    rope: &'a Rope,
    cursor: &'a (usize, usize),
    mode: &'a Mode,
}

impl<'a> CursorLine<'a> {
    pub fn new(rope: &'a Rope, cursor: &'a (usize, usize), mode: &'a Mode) -> Self {
        CursorLine { rope, cursor, mode }
    }
}

impl<'a> Widget for CursorLine<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let scroll_pos = self.cursor.1;
        let style = Style::default().fg(Color::White).bg(Color::Black);
        let cursor_style = self.mode.color();
        #[rustfmt::skip]
        let line = self.rope.line(scroll_pos).to_string().populate_fill_chars(); // panics
        let ratatui_line = Span::styled(line.to_string(), style);
        let ratatui_cursor = Span::styled(" ", cursor_style);
        buf.set_style(area, style);
        _ = buf.set_span(area.x, area.y, &ratatui_line, area.width);
        _ = buf.set_span(
            area.x + self.cursor.0 as u16,
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

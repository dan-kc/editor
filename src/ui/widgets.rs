use ratatui::{
    layout::Rect,
    style::Style,
    symbols::line::NORMAL,
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::app::Mode;

pub struct TextArea<'a> {
    buffer: &'a crate::buffer::Buffer,
}

impl<'a> TextArea<'a> {
    pub fn new(buffer: &'a crate::buffer::Buffer) -> Self {
        TextArea { buffer }
    }
}

impl<'a> Widget for TextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let text = &self.buffer.text;
        for (index, line) in text.lines().enumerate() {
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(0, index as u16, &ratatui_line, area.width)
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
        let text = self.mode.get_text().clone();
        let ratatui_line = Line::raw(text);
        _ = buf.set_line(0, area.y, &ratatui_line, area.width);
    }
}

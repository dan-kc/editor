use ratatui::{
    layout::Rect,
    style::Style,
    symbols::line::NORMAL,
    text::Line,
    widgets::{Block, Borders, Widget},
};
use ropey::Rope;

use crate::app::Mode;

pub struct TextArea<'a> {
    text: &'a Rope,
    scroll_pos: &'a u8,
}

impl<'a> TextArea<'a> {
    pub fn new(app: &'a crate::app::App) -> Self {
        TextArea {
            text: &app.buffer.text,
            scroll_pos: app.get_scroll_pos(),
        }
    }
}

impl<'a> Widget for TextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let scroll_pos = self.scroll_pos;
        let text = &self.text;
        for i in 0..buf.area.height - 1 {
            let line = text.line((*scroll_pos + i as u8) as usize);
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(0, i, &ratatui_line, area.width)
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

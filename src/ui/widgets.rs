use crate::app::Mode;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::Widget,
};
use ropey::Rope;

pub struct UpperTextArea<'a> {
    text: &'a Rope,
    scroll_pos: &'a u8,
}

impl<'a> UpperTextArea<'a> {
    pub fn new(app: &'a crate::app::App) -> Self {
        UpperTextArea {
            text: &app.buffer.text,
            scroll_pos: app.get_scroll_pos(),
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
        // Mock of top git info window
        for i in 0..top_info_line_count {
            let ratatui_line = Line::raw("------------");
            _ = buf.set_line(0, i, &ratatui_line, area.width)
        }

        let text_line_count = std::cmp::min(scroll_pos, area.height);
        for i in 0..text_line_count {
            let line_number = if scroll_pos >= area.height {
                (i + scroll_pos - area.height) as usize
            } else {
                i as usize
            };
            let line = self.text.line(line_number); // panics
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(0, i + top_info_line_count, &ratatui_line, area.width)
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
        let ratatui_line = Line::raw(text).style(Style::default().red());
        _ = buf.set_line(area.x, area.y, &ratatui_line, area.width);
    }
}

pub struct CursorLine<'a> {
    text: &'a Rope,
    scroll_pos: &'a u8,
}

impl<'a> CursorLine<'a> {
    pub fn new(text: &'a Rope, scroll_pos: &'a u8) -> Self {
        CursorLine { text, scroll_pos }
    }
}

impl<'a> Widget for CursorLine<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let text = &self.text;
        let scroll_pos = *self.scroll_pos as usize;
        let line = text.line(scroll_pos);
        let ratatui_line = Line::styled(line, Style::default().green());
        _ = buf.set_line(area.x, area.y, &ratatui_line, area.width)
    }
}

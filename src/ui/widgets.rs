use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Widget},
};

pub struct TextArea<'a> {
    // Custom widget properties
    buffer: &'a crate::buffer::Buffer,
}

impl<'a> TextArea<'a> {
    pub fn new(buffer: &'a crate::buffer::Buffer) -> Self {
        TextArea { buffer }
    }
}

// pub fn raw<T>(content: T) -> Line<'a>
// where
//     T: Into<Cow<'a, str>>,
// {
impl<'a> Widget for TextArea<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let text = &self.buffer.text;
        for (index, line) in text.lines().enumerate() {
            let ratatui_line = Line::raw(line);
            _ = buf.set_line(0, index as u16, &ratatui_line, area.width)
            // let area = ratatui::layout::Rect::new(0, 0, frame.size().width, 1);
        }

        // buf.set_line()
        // buf.set_string(
        //     area.left(),
        //     area.top(),
        //     &self.content,
        //     Style::default().fg(ratatui::style::Color::Red),
        // );
        // buf.set_line(0, 0', line, width)
    }
}

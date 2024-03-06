use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

use self::widgets::TextArea;
mod widgets;

/// Renders the widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        TextArea::new(&app.buffer),
        frame.size(),
    );
    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.counter
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title("Template")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan))
    //     .centered(),
    //     frame.size(),
    // );
}

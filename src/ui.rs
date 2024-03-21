use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;

use self::widgets::{CursorLine, StatusLine, UpperTextArea};

mod widgets;

pub fn render(app: &App, frame: &mut Frame) {
    let status_line_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.size());

    let upper_lower_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(status_line_layout[0]);

    frame.render_widget(StatusLine::new(&app.mode), status_line_layout[1]);
    frame.render_widget(UpperTextArea::new(app), upper_lower_layout[0]);
    #[rustfmt::skip]
    frame.render_widget(CursorLine::new(&app.buffer.text, app.get_scroll_pos()), upper_lower_layout[1]);
}

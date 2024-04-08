use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;

use self::widgets::{CursorLine, GitSummary, Logs, LowerTextArea, StatusLine, UpperTextArea};

mod widgets;

pub fn render(app: &App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.size());

    let upper_window_size = (frame.size().height - 2) / 2;
    let lower_window_size = frame.size().height - 2 - upper_window_size;

    let window_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Length(upper_window_size),
            Constraint::Length(1),
            Constraint::Length(lower_window_size),
        ])
        .split(outer_layout[0]);

    let scroll_pos = *app.scroll_pos() as u16;
    let upper_text_area_size = std::cmp::min(upper_window_size, scroll_pos);
    let git_summary_size = upper_window_size - upper_text_area_size;
    let upper_window_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Length(git_summary_size),
            Constraint::Length(upper_text_area_size),
        ])
        .split(window_layout[0]);

    let len_lines = app.buffer.rope.line_len() as u16;
    let lower_text_area_size = std::cmp::min(lower_window_size, len_lines - 1 - scroll_pos);
    let logs_size = lower_window_size - lower_text_area_size;

    let lower_window_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Length(lower_text_area_size),
            Constraint::Length(logs_size),
        ])
        .split(window_layout[2]);

    frame.render_widget(StatusLine::new(&app.mode), outer_layout[1]);
    frame.render_widget(GitSummary::new(app), upper_window_layout[0]);
    frame.render_widget(UpperTextArea::new(app), upper_window_layout[1]);
    #[rustfmt::skip]
    frame.render_widget(CursorLine::new(&app.buffer.rope, &app.cursor, &app.mode), window_layout[1]);
    frame.render_widget(LowerTextArea::new(app), lower_window_layout[0]);
    frame.render_widget(Logs::new(&app.logger), lower_window_layout[1]);
}

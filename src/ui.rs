use self::widgets::{
    CursorLine, GitSummary, Logs, LowerTextArea, MessageBlock, ModeBlock,
    UpperTextArea,
};
use crate::{app::App, logger::Logger};
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

mod widgets;

pub fn render(app: &App, logger: &Logger, frame: &mut Frame) {
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

    let scroll_pos = app.buffer.cursor.y as u16;
    let upper_text_area_size = std::cmp::min(upper_window_size, scroll_pos);
    let git_summary_size = upper_window_size - upper_text_area_size;
    let upper_window_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Length(git_summary_size),
            Constraint::Length(upper_text_area_size),
        ])
        .split(window_layout[0]);

    let len_lines = app.buffer.len_lines() as u16;
    let lower_text_area_size =
        std::cmp::min(lower_window_size, len_lines - 1 - scroll_pos);
    let logs_size = lower_window_size - lower_text_area_size;

    let lower_window_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Length(lower_text_area_size),
            Constraint::Length(logs_size),
        ])
        .split(window_layout[2]);

    let status_line_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Length(8), Constraint::Fill(1)])
        .split(outer_layout[1]);

    frame.render_widget(ModeBlock::new(app.mode()), status_line_layout[0]);
    frame.render_widget(
        MessageBlock::new(app.notifs().last()),
        status_line_layout[1],
    );
    frame.render_widget(GitSummary::new(app), upper_window_layout[0]);
    frame
        .render_widget(UpperTextArea::new(&app.buffer), upper_window_layout[1]);
    #[rustfmt::skip]
    frame.render_widget(CursorLine::new(&app.buffer, app.mode()), window_layout[1]);
    frame
        .render_widget(LowerTextArea::new(&app.buffer), lower_window_layout[0]);
    frame.render_widget(Logs::new(&logger), lower_window_layout[1]);
}

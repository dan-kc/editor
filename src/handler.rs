use crate::{
    app::{App, AppResult, IoResult, Message, Mode},
    logger::Level,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_events(app: &mut App) -> IoResult<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            if let Err(err) = handle_key_events(key_event, app) {
                app.push_msg(Message::from(&err))
            }
        }
        _ => {}
    };
    Ok(())
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult {
    match app.mode() {
        Mode::Normal => handle_normal_mode_key_events(key_event, app)?,
        Mode::Insert => handle_insert_mode_key_events(key_event, app)?,
        Mode::GoTo => handle_go_to_mode_key_events(key_event, app)?,
        Mode::Delete => handle_delete_mode_key_events(key_event, app)?,
    };
    Ok(())
}

pub fn handle_normal_mode_key_events(key_event: KeyEvent, app: &mut App) -> AppResult {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('i') => {
            app.enter_mode(Mode::Insert);
        }
        KeyCode::Char('g') => {
            app.enter_mode(Mode::GoTo);
        }
        KeyCode::Char('d') => {
            app.enter_mode(Mode::Delete);
        }
        KeyCode::Up => {
            app.move_up()?;
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Left => {
            app.move_left();
        }
        KeyCode::Right => {
            app.move_right();
        }
        KeyCode::Char('G') => {
            app.move_to_bottom();
        }
        KeyCode::Char('w') => {
            app.move_next_word_start();
        }
        KeyCode::Home => {
            app.move_start_line();
        }
        KeyCode::End => {
            app.move_end_line();
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_insert_mode_key_events(key_event: KeyEvent, app: &mut App) -> AppResult {
    match key_event.code {
        KeyCode::Up => {
            app.move_up()?;
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Char(char) => app.insert_char(char),
        KeyCode::Esc => {
            app.enter_mode(Mode::Normal);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_go_to_mode_key_events(key_event: KeyEvent, app: &mut App) -> AppResult {
    match key_event.code {
        KeyCode::Char('g') => {
            app.move_to_top();
            app.enter_mode(Mode::Normal)
        }
        KeyCode::Esc => {
            app.enter_mode(Mode::Normal);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_delete_mode_key_events(key_event: KeyEvent, app: &mut App) -> AppResult {
    match key_event.code {
        KeyCode::Char('d') => {
            app.delete_line();
            app.logger_mut()
                .log(Level::Info, String::from("deleted line"));
            app.enter_mode(Mode::Normal);
            app.move_up()?;
        }
        KeyCode::Esc => {
            app.enter_mode(Mode::Normal);
        }
        _ => {}
    }
    Ok(())
}

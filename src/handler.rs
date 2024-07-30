use crate::{
    app::{App, AppError, AppResult, IoResult, Mode, Notification},
    logger::{Level, Logger},
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};

#[derive(Default)]
pub struct Handler {
    pub logger: Logger,
    key_events: Vec<KeyEvent>,
    count: Option<usize>,
    pub prev_sequences: Vec<Box<[KeyEvent]>>,
    // alternate gray/black for each event. Not sure how
    // to display modifiers...
}

impl Handler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn listen(&mut self, app: &mut App) -> IoResult<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, app);
            }
            _ => {}
        };

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        if let Err(err) = match app.mode() {
            Mode::Normal => self.handle_normal_mode_key_event(key_event, app),
            Mode::Insert => self.handle_insert_mode_key_event(key_event, app),
            Mode::GoTo => self.handle_go_to_mode_key_event(key_event, app),
            Mode::Delete => self.handle_delete_mode_key_event(key_event, app),
        } {
            app.push_notif(Notification::from(&err))
        };
    }

    fn handle_normal_mode_key_event(
        &mut self,
        key_event: KeyEvent,
        app: &mut App,
    ) -> AppResult<()> {
        match key_event.code {
            KeyCode::Up => {
                let count = self.count.take().unwrap_or(1);
                app.move_up(count)?
            }
            KeyCode::Down => {
                let count = self.count.take().unwrap_or(1);
                app.move_down(count)?
            }
            KeyCode::Left => {
                let count = self.count.take().unwrap_or(1);
                app.move_left(count)?
            }
            KeyCode::Right => {
                let count = self.count.take().unwrap_or(1);
                app.move_right(count)
            }
            KeyCode::Char('G') => {
                let count = self.count.take();
                app.move_to_end_of_file()?;
                if count.is_some() {
                    return Err(AppError::CountRedundant);
                };
            }
            KeyCode::Char('w') => {
                let count = self.count.take().unwrap_or(1);
                app.move_next_word_start(count)?
            }
            KeyCode::Char('W') => {
                let count = self.count.take().unwrap_or(1);
                app.move_next_long_word_start(count)?
            }
            KeyCode::Char('e') => {
                let count = self.count.take().unwrap_or(1);
                app.move_next_word_end(count)?
            }
            KeyCode::Char('E') => {
                let count = self.count.take().unwrap_or(1);
                app.move_next_long_word_end(count)?
            }
            KeyCode::Char('b') => {
                let count = self.count.take().unwrap_or(1);
                app.move_prev_word_start(count)?
            }
            KeyCode::Char('B') => {
                let count = self.count.take().unwrap_or(1);
                app.move_prev_long_word_start(count)?
            }
            KeyCode::Home => {
                let count = self.count.take();
                app.move_start_line()?;
                if count.is_some() {
                    return Err(AppError::CountRedundant);
                };
            }
            KeyCode::End => {
                let count = self.count.take();
                app.move_end_line()?;
                if count.is_some() {
                    return Err(AppError::CountRedundant);
                };
            }
            KeyCode::Char('i') => {
                self.key_events = vec![key_event];
                let pos = app.buffer.cursor.into();
                if app.buffer.in_rope_bounds(pos)
                    || app.buffer.on_rope_tail(pos)
                {
                    app.enter_mode(Mode::Insert);
                    if self.count.take().is_some() {
                        return Err(AppError::CountRedundant);
                    };
                } else {
                    return Err(AppError::CursorOutOfBounds);
                }
            }
            KeyCode::Char('g') => {
                self.key_events = vec![key_event];
                app.enter_mode(Mode::GoTo);
                if self.count.take().is_some() {
                    return Err(AppError::CountRedundant);
                };
            }
            KeyCode::Char('d') => {
                self.key_events = vec![key_event];
                app.enter_mode(Mode::Delete);
                if self.count.take().is_some() {
                    return Err(AppError::CountRedundant);
                };
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                    // TODO: Save logs
                }
            }
            KeyCode::Char(numb) if numb.is_ascii_digit() => {
                self.add_count_digit(key_event);
            }
            _ => {
                self.reset_count();
                return Err(AppError::KeyUnmapped);
            }
        }

        Ok(())
    }

    /// Count will be zero and keys will be 'i' when insert mode is entered.
    fn handle_insert_mode_key_event(
        &mut self,
        key_event: KeyEvent,
        app: &mut App,
    ) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char(char) => {
                self.key_events.push(key_event);
                app.insert_char_before(char)?;
                app.move_right(1);
            }
            KeyCode::Enter => {
                self.key_events.push(key_event);
                app.insert_char_before('\n')?;
                app.move_down(1)?;
                app.move_start_line()?;
            }
            KeyCode::Esc => {
                self.flush_keys();
                let _ = app.move_left(1);
                app.enter_mode(Mode::Normal)
            }
            _ => return Err(AppError::KeyUnmapped),
        }

        Ok(())
    }

    /// Count will be zero and keys will be 'g' when insert mode is entered.
    fn handle_go_to_mode_key_event(
        &self,
        key_event: KeyEvent,
        app: &mut App,
    ) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('g') => {
                app.enter_mode(Mode::Normal);
                app.move_to_start_of_file()?;
            }
            KeyCode::Esc => {
                app.enter_mode(Mode::Normal);
            }
            _ => return Err(AppError::KeyUnmapped),
        };

        Ok(())
    }

    /// Count will be zero and keys will be 'd' when insert mode is entered.
    fn handle_delete_mode_key_event(
        &mut self,
        key_event: KeyEvent,
        app: &mut App,
    ) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('d') => {
                let count = self.count.take().unwrap_or(1);
                app.delete_lines(count)?;
                app.enter_mode(Mode::Normal);
                self.reset_count();
            }
            KeyCode::Char(numb) if numb.is_ascii_digit() => {
                self.add_count_digit(key_event);
            }
            KeyCode::Esc => {
                self.reset_keys();
                self.reset_count();
                app.enter_mode(Mode::Normal);
            }
            _ => return Err(AppError::KeyUnmapped),
        }

        Ok(())
    }

    fn reset_keys(&mut self) {
        self.key_events = vec![];
    }

    fn flush_keys(&mut self) {
        let taken_vec = std::mem::take(&mut self.key_events).into_boxed_slice();
        self.prev_sequences.push(taken_vec);
    }

    fn reset_count(&mut self) {
        self.count = None
    }

    fn add_count_digit(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char(digit) = key_event.code {
            let count = self.count.take().unwrap_or(0);
            let updated_count =
                count * 10 + digit.to_digit(10).unwrap() as usize;

            self.count = Some(updated_count);
            self.key_events.push(key_event);
        };
    }
}

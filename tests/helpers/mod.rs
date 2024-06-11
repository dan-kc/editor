use crossterm::event::{KeyCode, KeyEvent};
use editor::{app, handler};

#[derive(Default)]
enum MockFile {
    #[default]
    Basic,
    Empty,
    SingleLine,
    Sparse,
}

struct AppBuilder {
    mock_file: MockFile,
    keys: Vec<KeyEvent>,
}

impl AppBuilder {
    fn build(self) -> (app::App, handler::Handler) {
        todo!()
    }
    fn press_key(mut self, event: KeyEvent) -> Self {
        self.keys.push(event);

        self
    }
    fn new_from_file() -> Self {
        todo!()
    }
    fn new_default() -> Self {
        todo!()
    }
}

pub fn app_default() -> (app::App, handler::Handler) {
    AppBuilder::new_default().build()
}

pub fn app_with_cursor_at_end_of_first_line() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(KeyCode::End.into())
        .build()
}

pub fn app_with_cursor_at_end_of_last_line() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(KeyCode::Char('G').into())
        .build()
}

pub fn app_with_cursor_on_rope_tail() -> (app::App, handler::Handler) {
    todo!()
}

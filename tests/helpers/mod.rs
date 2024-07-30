use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use editor::{
    app::{self, Mode},
    buffer::Buffer,
    handler,
};

#[derive(Default)]
enum MockFile {
    #[default]
    Basic,
    Empty,
    SingleLine,
    Sparse,
}

fn init_app(file: MockFile) -> (app::App, handler::Handler) {
    let file_name: String = match file {
        MockFile::Basic => "tests/helpers/mock_files/basic.txt".into(),
        MockFile::Empty => "tests/helpers/mock_files/empty.txt".into(),
        MockFile::Sparse => "tests/helpers/mock_files/sparse.txt".into(),
        MockFile::SingleLine => {
            "tests/helpers/mock_files/single_line.txt".into()
        }
    };

    let buffer = Buffer::from_file(file_name).unwrap();
    (app::App::new(buffer), handler::Handler::new())
}

struct AppBuilder {
    mock_file: MockFile,
    keys: Vec<KeyEvent>,
}

impl AppBuilder {
    fn build(self) -> (app::App, handler::Handler) {
        let (mut app, mut handler) = init_app(self.mock_file);
        for event in self.keys {
            handler.handle_key_event(event, &mut app);
        }

        (app, handler)
    }

    fn press_key(mut self, event: KeyEvent) -> Self {
        self.keys.push(event);

        self
    }

    fn from_file(mock_file: MockFile) -> Self {
        Self {
            mock_file,
            keys: vec![],
        }
    }

    fn new_default() -> Self {
        Self {
            mock_file: MockFile::Basic,
            keys: vec![],
        }
    }
}

pub fn app_default() -> (app::App, handler::Handler) {
    AppBuilder::new_default().build()
}

pub fn app_with_cursor_on_end_of_first_line() -> (app::App, handler::Handler) {
    AppBuilder::new_default().press_key(END_KEY).build()
}

pub fn app_with_cursor_on_start_of_last_line() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(CAP_G_KEY)
        .press_key(HOME_KEY)
        .build()
}

pub fn app_with_cursor_on_end_of_file() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(CAP_G_KEY)
        .press_key(HOME_KEY)
        .build()
}

pub fn app_with_cursor_on_rope_tail() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(CAP_G_KEY)
        .press_key(RIGHT_KEY)
        .build()
}

pub fn app_with_cursor_on_newline_char() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(END_KEY)
        .press_key(RIGHT_KEY)
        .build()
}

pub fn app_with_cursor_out_of_rope_bounds() -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(END_KEY)
        .press_key(RIGHT_KEY)
        .press_key(RIGHT_KEY)
        .build()
}

pub fn app_with_empty_file() -> (app::App, handler::Handler) {
    AppBuilder::from_file(MockFile::Empty).build()
}

pub fn app_in_delete_mode_with_cursor_at_end_of_file(
) -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(CAP_G_KEY)
        .press_key(D_KEY)
        .build()
}

pub fn app_in_delete_mode_in_single_line_file() -> (app::App, handler::Handler)
{
    AppBuilder::from_file(MockFile::SingleLine)
        .press_key(D_KEY)
        .build()
}

pub fn app_in_delete_mode() -> (app::App, handler::Handler) {
    AppBuilder::new_default().press_key(D_KEY).build()
}

pub fn app_in_insert_mode() -> (app::App, handler::Handler) {
    AppBuilder::new_default().press_key(I_KEY).build()
}

pub fn app_in_insert_mode_with_empty_file() -> (app::App, handler::Handler) {
    AppBuilder::from_file(MockFile::Empty)
        .press_key(I_KEY)
        .build()
}

pub fn app_in_insert_mode_with_cursor_on_newline_char(
) -> (app::App, handler::Handler) {
    AppBuilder::new_default()
        .press_key(END_KEY)
        .press_key(RIGHT_KEY)
        .press_key(I_KEY)
        .build()
}

pub fn app_in_goto_mode() -> (app::App, handler::Handler) {
    AppBuilder::new_default().press_key(G_KEY).build()
}

pub fn app_with_empty_file_and_cursor_moved_right(
) -> (app::App, handler::Handler) {
    AppBuilder::from_file(MockFile::Empty)
        .press_key(RIGHT_KEY)
        .build()
}

// Key events
pub const RIGHT_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
pub const LEFT_KEY: KeyEvent = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
pub const UP_KEY: KeyEvent = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
pub const DOWN_KEY: KeyEvent = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
pub const ZERO_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE);
pub const THREE_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);
pub const FIVE_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE);
pub const I_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
pub const CAP_G_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);
pub const G_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
pub const D_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE);
pub const W_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
pub const A_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
pub const CAP_W_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE);
pub const E_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
pub const CAP_E_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('E'), KeyModifiers::NONE);
pub const B_KEY: KeyEvent =
    KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE);
pub const END_KEY: KeyEvent = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);
pub const HOME_KEY: KeyEvent = KeyEvent::new(KeyCode::Home, KeyModifiers::NONE);
pub const ESC_KEY: KeyEvent = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

// Notable positions
pub const END_OF_FIRST_LINE: (usize, usize) = (33, 0);
pub const START_OF_LAST_LINE: (usize, usize) = (0, 6);
pub const END_OF_LAST_LINE: (usize, usize) = (9, 6);

// Modes
pub const NORMAL: Mode = Mode::Normal;
pub const INSERT: Mode = Mode::Insert;
pub const GOTO: Mode = Mode::GoTo;
pub const DELETE: Mode = Mode::Delete;

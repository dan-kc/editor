use editor::{
    app::{self, IoResult, RunningState},
    buffer::Buffer,
    handler, tui,
};

fn main() -> IoResult<()> {
    let path = std::env::args().nth(1).expect("file name not found");
    let buffer = Buffer::from_file(path).expect("could not find file");
    let mut app = app::App::new(buffer);

    tui::install_panic_hook();
    let mut terminal = tui::init()?;
    let mut handler = handler::Handler::new();

    while app.running_state() == RunningState::Running {
        tui::draw(&mut terminal, &app, &handler.logger)?;
        handler.listen(&mut app)?; // blocks
    }

    tui::exit()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use editor::app::Mode;

    use super::*;

    #[allow(dead_code)]
    enum MockFile {
        Basic,
        Empty,
        Sparse,
        SingleLine,
    }

    fn init(file: MockFile) -> app::App {
        let file_name: String = match file {
            MockFile::Basic => "tests/mocks/basic.txt".into(),
            MockFile::Empty => "tests/mocks/empty.txt".into(),
            MockFile::Sparse => "tests/mocks/sparse.txt".into(),
            MockFile::SingleLine => "tests/mocks/single_line.txt".into(),
        };

        let buffer = Buffer::from_file(file_name).unwrap();
        app::App::new(buffer)
    }

    #[test]
    fn should_move_down() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let three_key_event =
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);

        handler.handle_key_event(down_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 1).into());

        handler.handle_key_event(three_key_event, &mut app);
        handler.handle_key_event(down_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 4).into());

        assert!(app.notifs().is_empty());
    }

    #[test]
    fn fail_move_down() {
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);

        handler.handle_key_event(down_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  already at bottom"
        )
    }

    #[test]
    fn should_move_up() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let cap_g_key_event =
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);
        let up_key_event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let two_key_event =
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);

        handler.handle_key_event(cap_g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (9, 6).into());

        handler.handle_key_event(up_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (9, 5).into());

        handler.handle_key_event(two_key_event, &mut app);
        handler.handle_key_event(up_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (9, 3).into());

        assert!(app.notifs().is_empty());
    }

    #[test]
    fn fail_move_up() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let up_key_event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);

        handler.handle_key_event(up_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  already at top"
        )
    }

    #[test]
    fn should_move_left() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        let left_key_event = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
        let five_key_event =
            KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE);
        let three_key_event =
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);

        handler.handle_key_event(five_key_event, &mut app);
        handler.handle_key_event(right_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (5, 0).into());

        handler.handle_key_event(left_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (4, 0).into());

        handler.handle_key_event(three_key_event, &mut app);
        handler.handle_key_event(left_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (1, 0).into());
    }

    #[test]
    fn fail_move_left() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let left_key_event = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);

        handler.handle_key_event(left_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  already leftmost"
        );
    }

    #[test]
    fn should_move_right() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        let three_key_event =
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);

        handler.handle_key_event(right_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (1, 0).into());

        handler.handle_key_event(three_key_event, &mut app);
        handler.handle_key_event(right_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (4, 0).into());

        assert!(app.notifs().is_empty());
    }

    #[test]
    fn should_move_start_of_line() {
        todo!();
    }

    #[test]
    fn fail_move_start_of_line() {
        todo!();
    }

    #[test]
    fn should_move_end_of_line() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);

        handler.handle_key_event(end_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (33, 0).into());
    }

    #[test]
    fn fail_move_end_of_line() {
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();

        let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);

        handler.handle_key_event(end_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no chars in line"
        );

        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        let cap_g_key_event =
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);
        handler.handle_key_event(cap_g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (11, 0).into());
        handler.handle_key_event(end_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (11, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  already at line end"
        );
    }

    #[test]
    fn should_move_end_of_file() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let cap_g_key_event =
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);

        handler.handle_key_event(cap_g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (9, 6).into());
    }

    #[test]
    fn fail_move_end_of_file() {
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();

        let cap_g_key_event =
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);

        handler.handle_key_event(cap_g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no chars in file"
        );
    }

    #[test]
    fn should_move_start_of_file() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let cap_g_key_event =
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);
        let g_key_event = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);

        handler.handle_key_event(cap_g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (9, 6).into());
        assert_eq!(app.mode(), Mode::Normal);

        handler.handle_key_event(g_key_event, &mut app);
        assert_eq!(app.mode(), Mode::GoTo);

        handler.handle_key_event(g_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(app.mode(), Mode::Normal);
        assert!(app.notifs().is_empty());
    }

    #[test]
    fn fail_move_start_of_file() {
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();

        let g_key_event = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);

        handler.handle_key_event(g_key_event, &mut app);
        handler.handle_key_event(g_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Normal);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no chars in file"
        );
    }

    #[test]
    fn should_enter_exit_insert_mode() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let i_key_event = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let esc_key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        handler.handle_key_event(i_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Insert);

        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Normal);
    }

    #[test]
    fn fail_enter_exit_insert_mode() {
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();

        let i_key_event = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);

        handler.handle_key_event(right_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (1, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  cursor out of bounds"
        );

        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        handler.handle_key_event(end_key_event, &mut app);
        handler.handle_key_event(right_key_event, &mut app);
        handler.handle_key_event(right_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);

        assert_eq!(app.buffer.cursor, (13, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  cursor out of bounds"
        );
    }

    #[test]
    fn should_enter_exit_go_to_mode() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let g_key_event = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let esc_key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        handler.handle_key_event(g_key_event, &mut app);
        assert_eq!(app.mode(), Mode::GoTo);

        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Normal);
    }

    #[test]
    fn should_enter_exit_delete_mode() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let d_key_event = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE);
        let esc_key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        handler.handle_key_event(d_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Delete);

        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.mode(), Mode::Normal);
    }

    #[test]
    fn should_move_next_word_start() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let w_key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let three_key_event =
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);
        let five_key_event =
            KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE);

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (2, 0).into());

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (3, 0).into());

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (10, 0).into());

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (14, 0).into());

        handler.handle_key_event(three_key_event, &mut app);
        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (18, 0).into());

        handler.handle_key_event(down_key_event, &mut app);
        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (20, 1).into());

        handler.handle_key_event(five_key_event, &mut app);
        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (36, 1).into());

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (36, 1).into());
    }

    #[test]
    fn fail_move_next_word_start() {
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        let w_key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);

        handler.handle_key_event(w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no more words in line"
        );
    }

    #[test]
    fn should_move_next_long_word_start() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);

        let cap_w_key_event =
            KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE);
        let two_key_event =
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);

        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (10, 0).into());

        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (14, 0).into());

        handler.handle_key_event(two_key_event, &mut app);
        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (21, 0).into());

        handler.handle_key_event(down_key_event, &mut app);
        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (24, 1).into());

        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (24, 1).into());
    }

    #[test]
    fn fail_move_next_long_word_start() {
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        let cap_w_key_event =
            KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE);

        handler.handle_key_event(cap_w_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no more words in line"
        );
    }

    #[test]
    fn should_move_next_word_end() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let e_key_event = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let six_key_event =
            KeyEvent::new(KeyCode::Char('6'), KeyModifiers::NONE);
        let two_key_event =
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);

        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (1, 0).into());

        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (2, 0).into());

        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (8, 0).into());

        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (11, 0).into());

        handler.handle_key_event(two_key_event, &mut app);
        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (16, 0).into());

        handler.handle_key_event(down_key_event, &mut app);
        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (19, 1).into());

        handler.handle_key_event(six_key_event, &mut app);
        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (36, 1).into());

        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (36, 1).into());
    }

    #[test]
    fn fail_move_next_word_end() {
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        let e_key_event = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        let two_key_event =
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);

        handler.handle_key_event(two_key_event, &mut app);
        handler.handle_key_event(e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no more words in line"
        )
    }

    #[test]
    fn should_move_next_long_word_end() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let cap_e_key_event =
            KeyEvent::new(KeyCode::Char('E'), KeyModifiers::NONE);
        let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let three_key_event =
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);

        handler.handle_key_event(cap_e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (8, 0).into());

        handler.handle_key_event(cap_e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (11, 0).into());

        handler.handle_key_event(down_key_event, &mut app);
        handler.handle_key_event(three_key_event, &mut app);
        handler.handle_key_event(cap_e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (36, 1).into());
    }

    #[test]
    fn fail_move_next_long_word_end() {
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();

        let cap_e_key_event =
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        let two_key_event =
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);

        handler.handle_key_event(two_key_event, &mut app);
        handler.handle_key_event(cap_e_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 0).into());
        assert_eq!(
            app.notifs().last().unwrap().to_string(),
            "  no more words in line"
        )
    }

    #[test]
    fn should_insert_text() {
        let mut app = init(MockFile::Basic);
        let mut handler = handler::Handler::new();

        let i_key_event = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let h_key_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let esc_key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);
        let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        let enter_key_event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);

        handler.handle_key_event(i_key_event, &mut app);
        assert!(app.notifs().is_empty());
        handler.handle_key_event(h_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (1, 0).into());

        // End of file.
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();
        handler.handle_key_event(end_key_event, &mut app);
        handler.handle_key_event(right_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(h_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(esc_key_event, &mut app);

        assert!(app.notifs().is_empty());
        assert_eq!(app.buffer.cursor, (13, 0).into());
        assert_eq!(app.buffer.line(0).unwrap().to_string(), "hiahetsaithehi");

        // Empty file.
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(h_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(esc_key_event, &mut app);

        assert!(app.notifs().is_empty());
        assert_eq!(app.buffer.cursor, (1, 0).into());
        assert_eq!(app.buffer.line(0).unwrap().to_string(), "hi");

        // Enter key.
        let mut app = init(MockFile::SingleLine);
        let mut handler = handler::Handler::new();
        handler.handle_key_event(right_key_event, &mut app);
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(enter_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 1).into());
        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 1).into());

        // Enter key empty file
        // Enter key.
        let mut app = init(MockFile::Empty);
        let mut handler = handler::Handler::new();
        handler.handle_key_event(i_key_event, &mut app);
        handler.handle_key_event(enter_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 1).into());
        handler.handle_key_event(h_key_event, &mut app);

        handler.handle_key_event(esc_key_event, &mut app);
        assert_eq!(app.buffer.cursor, (0, 1).into());
        assert_eq!(app.buffer.line(0).unwrap().to_string(), "\n");
        assert_eq!(app.buffer.line(1).unwrap().to_string(), "h");
    }
}

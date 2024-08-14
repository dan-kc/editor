use crate::helpers::{
    app_default, app_with_cursor_on_end_of_first_line,
    app_with_cursor_on_newline_char, app_with_cursor_out_of_rope_bounds,
    app_with_empty_file, app_with_empty_file_and_cursor_moved_right, FIVE_KEY,
    HOME_KEY,
};

#[test]
fn home_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(HOME_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn home_key_from_out_of_bounds_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_out_of_rope_bounds();

    handler.handle_key_event(HOME_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn home_key_from_newline_char_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_newline_char();

    handler.handle_key_event(HOME_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn home_key_from_line_start_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(HOME_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();

    assert_eq!(most_recent_notif, "  already at line start")
}

#[test]
fn home_key_on_empty_line_should_notify() {
    let (mut app, mut handler) = app_with_empty_file();

    handler.handle_key_event(HOME_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();

    assert_eq!(most_recent_notif, "  no chars in line")
}

#[test]
fn home_key_on_empty_line_should_move_cursor() {
    let (mut app, mut handler) = app_with_empty_file_and_cursor_moved_right();

    handler.handle_key_event(HOME_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn home_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(HOME_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn five_key_then_home_key_should_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(HOME_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  count redundant")
}

#[test]
fn five_key_then_home_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(HOME_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

use crate::helpers::{
    app_default, app_in_delete_mode, app_in_delete_mode_in_single_line_file,
    app_in_delete_mode_with_cursor_at_end_of_file, DOWN_KEY, D_KEY, FIVE_KEY,
    FOUR_KEY, THREE_KEY, ZERO_KEY,
};

#[test]
fn d_key_should_delete_one_line() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "T1r¢is massa sed tem8us soll+citudin.\n"
    )
}

#[test]
fn d_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn d_key_on_last_line_should_move_cursor() {
    let (mut app, mut handler) =
        app_in_delete_mode_with_cursor_at_end_of_file();

    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (9, 5).into())
}

#[test]
fn five_key_then_d_key_should_delete_lines() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "  d tem8us soll+citudin.\n"
    )
}

#[test]
fn zero_key_then_d_key_should_not_delete_lines() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(ZERO_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "Po:¢7i¢or l¢  a d.am soll!c7tudin.\n"
    )
}

#[test]
fn five_key_then_d_key_should_notify() {
    let (mut app, mut handler) =
        app_in_delete_mode_with_cursor_at_end_of_file();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no lines to delete"
    )
}

#[test]
fn five_key_then_d_key_should_not_delete_lines() {
    let (mut app, mut handler) =
        app_in_delete_mode_with_cursor_at_end_of_file();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "Po:¢7i¢or l¢  a d.am soll!c7tudin.\n"
    )
}

#[test]
fn five_key_then_d_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn d_key_should_not_notify() {
    let (mut app, mut handler) = app_in_delete_mode_in_single_line_file();

    handler.handle_key_event(D_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn d_key_then_d_key_then_d_key_should_not_notify() {
    let (mut app, mut handler) = app_in_delete_mode_in_single_line_file();

    handler.handle_key_event(D_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no lines to delete"
    )
}

#[test]
fn subsequent_deletes() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(DOWN_KEY, &mut app);

    handler.handle_key_event(D_KEY, &mut app);
    handler.handle_key_event(FOUR_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);
    dbg!(app.buffer.cursor);

    handler.handle_key_event(D_KEY, &mut app);
    handler.handle_key_event(D_KEY, &mut app);
    assert_eq!(app.buffer.cursor, (0, 1).into())
}

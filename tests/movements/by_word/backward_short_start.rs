use crate::helpers::{
    app_default, app_with_cursor_on_end_of_first_line,
    app_with_cursor_on_indented_line, app_with_cursor_on_newline_char,
    app_with_cursor_out_of_rope_bounds, B_KEY, FIVE_KEY, RIGHT_KEY, THREE_KEY,
    W_KEY,
};

#[test]
fn b_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (28, 0).into())
}

#[test]
fn b_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn b_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no more words in line"
    )
}

#[test]
fn b_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(B_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn three_key_then_b_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (26, 0).into())
}

#[test]
fn three_key_then_b_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn three_key_then_b_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no more words in line"
    )
}

#[test]
fn three_key_then_b_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn b_key_should_move_cursor_from_newline_char() {
    let (mut app, mut handler) = app_with_cursor_on_newline_char();

    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (33, 0).into())
}

#[test]
fn b_key_should_move_cursor_from_out_of_bounds() {
    let (mut app, mut handler) = app_with_cursor_on_newline_char();

    handler.handle_key_event(RIGHT_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (33, 0).into())
}

#[test]
fn five_key_then_b_key_should_move_cursor_from_out_of_bounds() {
    let (mut app, mut handler) = app_with_cursor_on_newline_char();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(RIGHT_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (29, 0).into())
}

#[test]
fn five_key_then_five_key_then_b_key_should_move_cursor_from_out_of_bounds_on_empty_line(
) {
    let (mut app, mut handler) = app_with_cursor_out_of_rope_bounds();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (33, 0).into())
}

#[test]
fn five_key_then_five_key_then_b_key_should_notify_from_out_of_bounds_on_empty_line(
) {
    let (mut app, mut handler) = app_with_cursor_out_of_rope_bounds();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no more words in line"
    )
}

#[test]
fn five_key_five_key_b_key_should_notify_from_end_of_indented_line() {
    let (mut app, mut handler) = app_with_cursor_on_indented_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(W_KEY, &mut app);

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (7, 5).into())
}

#[test]
fn b_key_b_key_b_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_indented_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(W_KEY, &mut app);

    handler.handle_key_event(B_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);
    handler.handle_key_event(B_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (2, 5).into())
}

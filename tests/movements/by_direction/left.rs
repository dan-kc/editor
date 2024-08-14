use crate::helpers::{
    app_default, app_with_cursor_on_end_of_first_line, END_OF_FIRST_LINE,
    FIVE_KEY, LEFT_KEY,
};

#[test]
fn left_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(LEFT_KEY, &mut app);

    assert_eq!(
        app.buffer.cursor,
        (END_OF_FIRST_LINE.0 - 1, END_OF_FIRST_LINE.1).into()
    )
}

#[test]
fn left_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(LEFT_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn left_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(LEFT_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  already leftmost")
}

#[test]
fn left_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(LEFT_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn five_key_then_left_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(LEFT_KEY, &mut app);

    assert_eq!(
        app.buffer.cursor,
        (END_OF_FIRST_LINE.0 - 5, END_OF_FIRST_LINE.1).into()
    )
}

#[test]
fn five_key_then_left_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(LEFT_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn five_key_then_left_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(LEFT_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  already leftmost")
}

#[test]
fn five_key_then_left_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(LEFT_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

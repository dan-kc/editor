use crate::helpers::{app_default, FIVE_KEY, RIGHT_KEY};

#[test]
fn right_key_should_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(RIGHT_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (1, 0).into())
}

#[test]
fn right_key_should_not_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(RIGHT_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn five_key_then_right_key_should_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(RIGHT_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (5, 0).into())
}

#[test]
fn five_key_then_right_key_should_not_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(RIGHT_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

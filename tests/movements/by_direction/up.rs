use crate::helpers::{app_default, app_with_cursor_on_start_of_last_line, FIVE_KEY, START_OF_LAST_LINE, UP_KEY};

#[test]
fn up_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_start_of_last_line();

    handler.handle_key_event(UP_KEY, &mut app);

    assert_eq!(
        app.buffer.cursor,
        (START_OF_LAST_LINE.0, START_OF_LAST_LINE.1 - 1).into()
    )
}

#[test]
fn up_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(UP_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn up_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(UP_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  already at top")
}

#[test]
fn up_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_start_of_last_line();

    handler.handle_key_event(UP_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn five_key_then_up_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_start_of_last_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(UP_KEY, &mut app);

    assert_eq!(
        app.buffer.cursor,
        (START_OF_LAST_LINE.0, START_OF_LAST_LINE.1 - 5).into()
    )
}

#[test]
fn five_key_then_up_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(UP_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn five_key_then_up_key_should_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(UP_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  already at top")
}

#[test]
fn five_key_then_up_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_start_of_last_line();

    handler.handle_key_event(FIVE_KEY, &mut app);
    handler.handle_key_event(UP_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

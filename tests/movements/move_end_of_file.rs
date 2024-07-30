use crate::helpers::{
    app_default, app_with_empty_file,
    app_with_empty_file_and_cursor_moved_right, CAP_G_KEY, END_OF_LAST_LINE,
};

#[test]
fn cap_g_key_should_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(CAP_G_KEY, &mut app);

    assert_eq!(app.buffer.cursor, END_OF_LAST_LINE.into())
}

#[test]
fn cap_g_should_not_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(CAP_G_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn cap_g_key_on_empty_file_should_move_cursor() {
    let (mut app, mut handler) = app_with_empty_file_and_cursor_moved_right();

    handler.handle_key_event(CAP_G_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn cap_g_key_on_empty_file_should_notify() {
    let (mut app, mut handler) = app_with_empty_file();

    handler.handle_key_event(CAP_G_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  no chars in file")
}

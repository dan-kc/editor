use crate::helpers::{
    app_with_cursor_on_end_of_file, app_with_empty_file,
    app_with_empty_file_and_cursor_moved_right, G_KEY,
};

#[test]
fn g_key_then_g_key_should_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_file();

    handler.handle_key_event(G_KEY, &mut app);
    handler.handle_key_event(G_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn g_key_then_g_key_should_not_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_file();

    handler.handle_key_event(G_KEY, &mut app);
    handler.handle_key_event(G_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn g_key_then_g_key_on_empty_file_should_move_cursor() {
    let (mut app, mut handler) = app_with_empty_file_and_cursor_moved_right();

    handler.handle_key_event(G_KEY, &mut app);
    handler.handle_key_event(G_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (0, 0).into())
}

#[test]
fn g_key_then_g_key_on_empty_file_should_notify() {
    let (mut app, mut handler) = app_with_empty_file();

    handler.handle_key_event(G_KEY, &mut app);
    handler.handle_key_event(G_KEY, &mut app);

    let most_recent_notif = app.notifs().last().unwrap().to_string();
    assert_eq!(most_recent_notif, "  no chars in file")
}

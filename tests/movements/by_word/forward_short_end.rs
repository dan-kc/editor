use crate::helpers::{
    app_default, app_with_cursor_on_end_of_first_line, THREE_KEY, E_KEY,
};

#[test]
fn e_key_should_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (1, 0).into())
}

#[test]
fn e_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (33, 0).into())
}

#[test]
fn e_key_should_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no more words in line"
    )
}

#[test]
fn e_key_should_not_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(E_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

#[test]
fn three_key_then_e_key_should_move_cursor() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (3, 0).into())
}

#[test]
fn three_key_then_e_key_should_not_move_cursor() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.buffer.cursor, (33, 0).into())
}

#[test]
fn three_key_then_e_key_should_notify() {
    let (mut app, mut handler) = app_with_cursor_on_end_of_first_line();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(
        app.notifs().last().unwrap().to_string(),
        "  no more words in line"
    )
}

#[test]
fn three_key_then_e_key_should_not_notify() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(THREE_KEY, &mut app);
    handler.handle_key_event(E_KEY, &mut app);

    assert!(app.notifs().is_empty())
}

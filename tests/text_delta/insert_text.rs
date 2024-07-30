use crate::helpers::{
    app_in_insert_mode, app_in_insert_mode_with_cursor_on_newline_char,
    app_in_insert_mode_with_empty_file, A_KEY,
};

#[test]
fn a_key_should_insert_text_to_buffer() {
    let (mut app, mut handler) = app_in_insert_mode();

    handler.handle_key_event(A_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "aPo:¢7i¢or l¢  a d.am soll!c7tudin.\n"
    );
}

#[test]
fn a_key_on_newline_char_should_insert_text_to_buffer() {
    let (mut app, mut handler) =
        app_in_insert_mode_with_cursor_on_newline_char();

    handler.handle_key_event(A_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "Po:¢7i¢or l¢  a d.am soll!c7tudin.a\n"
    );
}

#[test]
fn a_key_in_empty_file_should_insert_text_to_buffer() {
    let (mut app, mut handler) = app_in_insert_mode_with_empty_file();

    handler.handle_key_event(A_KEY, &mut app);

    assert_eq!(
        app.buffer.line(0).unwrap().to_string(),
        "a"
    );
}


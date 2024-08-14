use editor::app::Mode;

use crate::helpers::{app_in_goto_mode, CAP_E_KEY, ESC_KEY, E_KEY, G_KEY};

#[test]
fn esc_should_enter_normal() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(ESC_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

#[test]
fn g_should_enter_normal() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(G_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

#[test]
fn e_should_enter_normal() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

#[test]
fn cap_e_should_enter_normal() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(CAP_E_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

#[test]
fn e_should_enter_normal_from_new_line_char() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(E_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

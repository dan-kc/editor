use editor::app::Mode;

use crate::helpers::{app_in_delete_mode, D_KEY, ESC_KEY};

#[test]
fn esc_key_should_enter_normal() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(ESC_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

#[test]
fn d_key_should_enter_normal() {
    let (mut app, mut handler) = app_in_delete_mode();

    handler.handle_key_event(D_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

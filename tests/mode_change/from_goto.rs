use editor::app::Mode;

use crate::helpers::{app_in_goto_mode, ESC_KEY};

#[test]
fn esc_key_should_enter_normal() {
    let (mut app, mut handler) = app_in_goto_mode();

    handler.handle_key_event(ESC_KEY, &mut app);

    assert_eq!(app.mode(), Mode::Normal)
}

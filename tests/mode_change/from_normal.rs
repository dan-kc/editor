use crate::helpers::{
    app_default, app_with_cursor_on_rope_tail,
    app_with_cursor_out_of_rope_bounds, INSERT, I_KEY, NORMAL,
};

#[test]
fn should_enter_insert() {
    let (mut app, mut handler) = app_default();

    handler.handle_key_event(I_KEY, &mut app);

    assert_eq!(app.mode(), INSERT);
}

#[test]
fn i_key_from_rope_tail_should_enter_insert() {
    let (mut app, mut handler) = app_with_cursor_on_rope_tail();

    handler.handle_key_event(I_KEY, &mut app);

    assert_eq!(app.mode(), INSERT);
}

#[test]
fn i_key_from_out_of_rope_bounds_should_not_enter_insert() {
    let (mut app, mut handler) = app_with_cursor_out_of_rope_bounds();

    handler.handle_key_event(I_KEY, &mut app);

    assert_eq!(app.mode(), NORMAL);
}
//
// #[test]
// fn five_key_then_i_key_should_enter_insert() {
//     todo!()
// }
//
// #[test]
// fn five_key_then_i_key_should_notify() {
//     todo!()
// }
//
// #[test]
// fn d_key_should_enter_delete() {
//     todo!()
// }
//
// #[test]
// fn d_key_in_empty_file_should_notify() {
//     todo!()
// }
//
// #[test]
// fn d_key_in_empty_file_should_not_enter_delete() {
//     todo!()
// }
//
// #[test]
// fn g_key_should_enter_go_to() {
//     todo!()
// }

mod helpers;
mod movements {
    mod move_right;
}

// mod move_right;

// mod tests {
//
//     mod movements {
//
//         mod move_left {
//             use crate::common::app_with_cursor_at_end_of_first_line;
//
//
//             #[test]
//             fn left_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert_eq!(
//                     app.buffer.cursor,
//                     (END_OF_FIRST_LINE.0 - 1, END_OF_FIRST_LINE.1).into()
//                 )
//             }
//
//             #[test]
//             fn left_key_should_not_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, (0, 0).into())
//             }
//
//             #[test]
//             fn left_key_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already leftmost")
//             }
//
//             #[test]
//             fn left_key_should_not_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn five_key_then_left_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert_eq!(
//                     app.buffer.cursor,
//                     (END_OF_FIRST_LINE.0 - 5, END_OF_FIRST_LINE.1).into()
//                 )
//             }
//
//             #[test]
//             fn five_key_then_left_key_should_not_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, (0, 0).into())
//             }
//
//             #[test]
//             fn five_key_then_left_key_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already leftmost")
//             }
//
//             #[test]
//             fn five_key_then_left_key_should_not_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(LEFT_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//         }
//
//         mod move_up {
//             use crate::tests::{
//                 app_default, app_with_cursor_at_start_of_last_line, FIVE_KEY,
//                 START_OF_LAST_LINE, UP_KEY,
//             };
//
//             #[test]
//             fn up_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert_eq!(
//                     app.buffer.cursor,
//                     (START_OF_LAST_LINE.0, START_OF_LAST_LINE.1 - 1).into()
//                 )
//             }
//
//             #[test]
//             fn up_key_should_not_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, (0, 0).into())
//             }
//
//             #[test]
//             fn up_key_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already at top")
//             }
//
//             #[test]
//             fn up_key_should_not_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn five_key_then_up_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert_eq!(
//                     app.buffer.cursor,
//                     (START_OF_LAST_LINE.0, START_OF_LAST_LINE.1 - 5).into()
//                 )
//             }
//
//             #[test]
//             fn five_key_then_up_key_should_not_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, (0, 0).into())
//             }
//
//             #[test]
//             fn five_key_then_up_key_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already at top")
//             }
//
//             #[test]
//             fn five_key_then_up_key_should_not_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(UP_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//         }
//
//         mod move_down {
//             use crate::tests::{
//                 app_default, app_with_cursor_at_start_of_last_line, DOWN_KEY,
//                 FIVE_KEY, START_OF_FIRST_LINE, START_OF_LAST_LINE,
//             };
//
//             #[test]
//             fn down_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, (0, 1).into())
//             }
//
//             #[test]
//             fn down_key_should_not_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_LAST_LINE.into())
//             }
//
//             #[test]
//             fn down_key_should_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already at bottom")
//             }
//
//             #[test]
//             fn down_key_should_not_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn five_key_then_down_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert_eq!(
//                     app.buffer.cursor,
//                     (START_OF_FIRST_LINE.0, START_OF_FIRST_LINE.1 + 5).into()
//                 )
//             }
//
//             #[test]
//             fn five_key_then_down_key_should_not_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_LAST_LINE.into())
//             }
//
//             #[test]
//             fn five_key_then_down_key_should_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_start_of_last_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  already at bottom")
//             }
//
//             #[test]
//             fn five_key_then_down_key_should_not_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(DOWN_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//         }
//
//         mod move_end_of_line {
//             use crate::tests::{
//                 app_default, app_with_cursor_at_end_of_first_line,
//                 app_with_cursor_on_newline_char,
//                 app_with_cursor_out_of_rope_bounds, app_with_empty_file,
//                 app_with_empty_file_and_with_cursor_moved_right, END_KEY,
//                 END_OF_FIRST_LINE, FIVE_KEY, START_OF_FIRST_LINE,
//             };
//
//             #[test]
//             fn end_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, END_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn end_key_from_out_of_bounds_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_out_of_rope_bounds();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, END_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn end_key_from_newline_char_should_move_cursor() {
//                 let (mut app, mut handler) = app_with_cursor_on_newline_char();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, END_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn end_key_from_line_end_should_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//
//                 assert_eq!(most_recent_notif, "  already at line end")
//             }
//
//             #[test]
//             fn end_key_on_empty_line_should_notify() {
//                 let (mut app, mut handler) = app_with_empty_file();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//
//                 assert_eq!(most_recent_notif, "  no chars in line")
//             }
//
//             #[test]
//             fn end_key_on_empty_line_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_empty_file_and_with_cursor_moved_right();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn end_key_should_not_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn five_key_then_end_key_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  count redundant")
//             }
//
//             #[test]
//             fn five_key_then_end_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(END_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, END_OF_FIRST_LINE.into())
//             }
//         }
//
//         mod move_start_of_line {
//             use crate::tests::{
//                 app_default, app_with_cursor_at_end_of_first_line,
//                 app_with_cursor_on_newline_char,
//                 app_with_cursor_out_of_rope_bounds, app_with_empty_file,
//                 app_with_empty_file_and_with_cursor_moved_right, FIVE_KEY,
//                 HOME_KEY, START_OF_FIRST_LINE,
//             };
//
//             #[test]
//             fn home_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn home_key_from_out_of_bounds_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_out_of_rope_bounds();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn home_key_from_newline_char_should_move_cursor() {
//                 let (mut app, mut handler) = app_with_cursor_on_newline_char();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn home_key_from_line_start_should_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//
//                 assert_eq!(most_recent_notif, "  already at line start")
//             }
//
//             #[test]
//             fn home_key_on_empty_line_should_notify() {
//                 let (mut app, mut handler) = app_with_empty_file();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//
//                 assert_eq!(most_recent_notif, "  no chars in line")
//             }
//
//             #[test]
//             fn home_key_on_empty_line_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_empty_file_and_with_cursor_moved_right();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn home_key_should_not_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn five_key_then_home_key_should_notify() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  count redundant")
//             }
//
//             #[test]
//             fn five_key_then_home_key_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_at_end_of_first_line();
//
//                 handler.handle_key_event(FIVE_KEY, &mut app);
//                 handler.handle_key_event(HOME_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//         }
//
//         mod move_end_of_file {
//             use crate::tests::{
//                 app_default, app_with_empty_file,
//                 app_with_empty_file_and_with_cursor_moved_right, CAP_G_KEY,
//                 END_OF_LAST_LINE, START_OF_FIRST_LINE,
//             };
//
//             #[test]
//             fn cap_g_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(CAP_G_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, END_OF_LAST_LINE.into())
//             }
//
//             #[test]
//             fn cap_g_should_not_notify() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(CAP_G_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn cap_g_key_on_empty_file_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_empty_file_and_with_cursor_moved_right();
//
//                 handler.handle_key_event(CAP_G_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn cap_g_key_on_empty_file_should_notify() {
//                 let (mut app, mut handler) = app_with_empty_file();
//
//                 handler.handle_key_event(CAP_G_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  no chars in file")
//             }
//         }
//
//         mod move_start_of_file {
//             use crate::tests::{
//                 app_with_cursor_at_end_of_file, app_with_empty_file,
//                 app_with_empty_file_and_with_cursor_moved_right, G_KEY,
//                 START_OF_FIRST_LINE,
//             };
//
//             #[test]
//             fn g_key_then_g_key_should_move_cursor() {
//                 let (mut app, mut handler) = app_with_cursor_at_end_of_file();
//
//                 handler.handle_key_event(G_KEY, &mut app);
//                 handler.handle_key_event(G_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn g_key_then_g_key_should_not_notify() {
//                 let (mut app, mut handler) = app_with_cursor_at_end_of_file();
//
//                 handler.handle_key_event(G_KEY, &mut app);
//                 handler.handle_key_event(G_KEY, &mut app);
//
//                 assert!(app.notifs().is_empty())
//             }
//
//             #[test]
//             fn g_key_then_g_key_on_empty_file_should_move_cursor() {
//                 let (mut app, mut handler) =
//                     app_with_empty_file_and_with_cursor_moved_right();
//
//                 handler.handle_key_event(G_KEY, &mut app);
//                 handler.handle_key_event(G_KEY, &mut app);
//
//                 assert_eq!(app.buffer.cursor, START_OF_FIRST_LINE.into())
//             }
//
//             #[test]
//             fn g_key_then_g_key_on_empty_file_should_notify() {
//                 let (mut app, mut handler) = app_with_empty_file();
//
//                 handler.handle_key_event(G_KEY, &mut app);
//                 handler.handle_key_event(G_KEY, &mut app);
//
//                 let most_recent_notif =
//                     app.notifs().last().unwrap().to_string();
//                 assert_eq!(most_recent_notif, "  no chars in file")
//             }
//         }
//     }
//
//     mod mode_change {
//         mod from_normal {
//
//             use crate::tests::{
//                 app_default, app_with_cursor_at_rope_tail,
//                 app_with_cursor_out_of_rope_bounds, INSERT, I_KEY, NORMAL,
//             };
//
//             #[test]
//             fn should_enter_insert() {
//                 let (mut app, mut handler) = app_default();
//
//                 handler.handle_key_event(I_KEY, &mut app);
//
//                 assert_eq!(app.mode(), INSERT);
//             }
//
//             #[test]
//             fn i_key_from_rope_tail_should_enter_insert() {
//                 let (mut app, mut handler) = app_with_cursor_at_rope_tail();
//
//                 handler.handle_key_event(I_KEY, &mut app);
//
//                 assert_eq!(app.mode(), INSERT);
//             }
//
//             #[test]
//             fn i_key_from_out_of_rope_bounds_should_not_enter_insert() {
//                 let (mut app, mut handler) =
//                     app_with_cursor_out_of_rope_bounds();
//
//                 handler.handle_key_event(I_KEY, &mut app);
//
//                 assert_eq!(app.mode(), NORMAL);
//             }
//
//             #[test]
//             fn five_key_then_i_key_should_enter_insert() {
//                 todo!()
//             }
//
//             #[test]
//             fn five_key_then_i_key_should_notify() {
//                 todo!()
//             }
//
//             #[test]
//             fn d_key_should_enter_delete() {
//                 todo!()
//             }
//
//             #[test]
//             fn d_key_in_empty_file_should_notify() {
//                 todo!()
//             }
//
//             #[test]
//             fn d_key_in_empty_file_should_not_enter_delete() {
//                 todo!()
//             }
//
//             #[test]
//             fn g_key_should_enter_go_to() {
//                 todo!()
//             }
//         }
//
//         mod from_insert {
//             #[test]
//             fn esc_key_should_enter_normal() {
//                 todo!()
//             }
//         }
//         mod from_delete {
//             #[test]
//             fn esc_key_should_enter_normal() {
//                 todo!()
//             }
//         }
//         mod from_go_to {
//             #[test]
//             fn esc_key_should_enter_normal() {
//                 todo!()
//             }
//         }
//     }
//
//     //
//     // #[test]
//     // fn should_move_next_word_start() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let w_key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
//     //     let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
//     //     let three_key_event =
//     //         KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);
//     //     let five_key_event =
//     //         KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (2, 0).into());
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (3, 0).into());
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (10, 0).into());
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (14, 0).into());
//     //
//     //     handler.handle_key_event(three_key_event, &mut app);
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (18, 0).into());
//     //
//     //     handler.handle_key_event(down_key_event, &mut app);
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (20, 1).into());
//     //
//     //     handler.handle_key_event(five_key_event, &mut app);
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (36, 1).into());
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (36, 1).into());
//     // }
//     //
//     // #[test]
//     // fn fail_move_next_word_start() {
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let w_key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 0).into());
//     //     assert_eq!(
//     //         app.notifs().last().unwrap().to_string(),
//     //         "  no more words in line"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn should_move_next_long_word_start() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //     let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
//     //
//     //     let cap_w_key_event =
//     //         KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE);
//     //     let KeyCode::Char('2').into() =
//     //         KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (10, 0).into());
//     //
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (14, 0).into());
//     //
//     //     handler.handle_key_event(KeyCode::Char('2').into(), &mut app);
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (21, 0).into());
//     //
//     //     handler.handle_key_event(down_key_event, &mut app);
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (24, 1).into());
//     //
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (24, 1).into());
//     // }
//     //
//     // #[test]
//     // fn fail_move_next_long_word_start() {
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let cap_w_key_event =
//     //         KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(cap_w_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 0).into());
//     //     assert_eq!(
//     //         app.notifs().last().unwrap().to_string(),
//     //         "  no more words in line"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn should_move_next_word_end() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let e_key_event = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
//     //     let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
//     //     let six_key_event =
//     //         KeyEvent::new(KeyCode::Char('6'), KeyModifiers::NONE);
//     //     let KeyCode::Char('2').into() =
//     //         KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (1, 0).into());
//     //
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (2, 0).into());
//     //
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (8, 0).into());
//     //
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (11, 0).into());
//     //
//     //     handler.handle_key_event(KeyCode::Char('2').into(), &mut app);
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (16, 0).into());
//     //
//     //     handler.handle_key_event(down_key_event, &mut app);
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (19, 1).into());
//     //
//     //     handler.handle_key_event(six_key_event, &mut app);
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (36, 1).into());
//     //
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (36, 1).into());
//     // }
//     //
//     // #[test]
//     // fn fail_move_next_word_end() {
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let e_key_event = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
//     //     let KeyCode::Char('2').into() =
//     //         KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(KeyCode::Char('2').into(), &mut app);
//     //     handler.handle_key_event(e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 0).into());
//     //     assert_eq!(
//     //         app.notifs().last().unwrap().to_string(),
//     //         "  no more words in line"
//     //     )
//     // }
//     //
//     // #[test]
//     // fn should_move_next_long_word_end() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let cap_e_key_event =
//     //         KeyEvent::new(KeyCode::Char('E'), KeyModifiers::NONE);
//     //     let down_key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
//     //     let three_key_event =
//     //         KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(cap_e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (8, 0).into());
//     //
//     //     handler.handle_key_event(cap_e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (11, 0).into());
//     //
//     //     handler.handle_key_event(down_key_event, &mut app);
//     //     handler.handle_key_event(three_key_event, &mut app);
//     //     handler.handle_key_event(cap_e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (36, 1).into());
//     // }
//     //
//     // #[test]
//     // fn fail_move_next_long_word_end() {
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let cap_e_key_event =
//     //         KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
//     //     let KeyCode::Char('2').into() =
//     //         KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(KeyCode::Char('2').into(), &mut app);
//     //     handler.handle_key_event(cap_e_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 0).into());
//     //     assert_eq!(
//     //         app.notifs().last().unwrap().to_string(),
//     //         "  no more words in line"
//     //     )
//     // }
//     //
//     // #[test]
//     // fn should_move_prev_word_start() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let b_key_event = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE);
//     //     let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);
//     //     let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
//     //     let four_key_event =
//     //         KeyEvent::new(KeyCode::Char('4'), KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(end_key_event, &mut app);
//     //     handler.handle_key_event(four_key_event, &mut app);
//     //     handler.handle_key_event(right_key_event, &mut app);
//     //     handler.handle_key_event(b_key_event, &mut app);
//     //     assert!(app.notifs().is_empty());
//     //     assert_eq!(app.buffer.cursor, (33, 0).into());
//     //     handler.handle_key_event(b_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (28, 0).into());
//     //
//     //     handler.handle_key_event(four_key_event, &mut app);
//     //     handler.handle_key_event(b_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (21, 0).into());
//     //
//     //     assert!(app.notifs().is_empty());
//     // }
//     //
//     // #[test]
//     // fn should_insert_text() {
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //
//     //     let i_key_event = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
//     //     let h_key_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
//     //     let esc_key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
//     //     let end_key_event = KeyEvent::new(KeyCode::End, KeyModifiers::NONE);
//     //     let right_key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
//     //     let enter_key_event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
//     //
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     assert!(app.notifs().is_empty());
//     //     handler.handle_key_event(h_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (1, 0).into());
//     //
//     //     // End of file.
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //     handler.handle_key_event(end_key_event, &mut app);
//     //     handler.handle_key_event(right_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(h_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //
//     //     assert!(app.notifs().is_empty());
//     //     assert_eq!(app.buffer.cursor, (13, 0).into());
//     //     assert_eq!(app.buffer.line(0).unwrap().to_string(), "hiahetsaithehi");
//     //
//     //     // On newline char
//     //     let mut app = init_app(MockFile::Basic);
//     //     let mut handler = handler::Handler::new();
//     //     handler.handle_key_event(end_key_event, &mut app);
//     //     handler.handle_key_event(right_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(h_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //     assert!(app.notifs().is_empty());
//     //     assert_eq!(app.buffer.cursor, (35, 0).into());
//     //     assert_eq!(
//     //         app.buffer.line(0).unwrap().to_string(),
//     //         "Po:¢ti¢or l¢  a d.am soll!c7tudin.hi\n"
//     //     );
//     //
//     //     // Empty file.
//     //     let mut app = init_app(MockFile::Empty);
//     //     let mut handler = handler::Handler::new();
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(h_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //
//     //     assert!(app.notifs().is_empty());
//     //     assert_eq!(app.buffer.cursor, (1, 0).into());
//     //     assert_eq!(app.buffer.line(0).unwrap().to_string(), "hi");
//     //
//     //     // Enter key.
//     //     let mut app = init_app(MockFile::SingleLine);
//     //     let mut handler = handler::Handler::new();
//     //     handler.handle_key_event(right_key_event, &mut app);
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(enter_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 1).into());
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 1).into());
//     //
//     //     // Enter key empty file
//     //     // Enter key.
//     //     let mut app = init_app(MockFile::Empty);
//     //     let mut handler = handler::Handler::new();
//     //     handler.handle_key_event(i_key_event, &mut app);
//     //     handler.handle_key_event(enter_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 1).into());
//     //     handler.handle_key_event(h_key_event, &mut app);
//     //
//     //     handler.handle_key_event(esc_key_event, &mut app);
//     //     assert_eq!(app.buffer.cursor, (0, 1).into());
//     //     assert_eq!(app.buffer.line(0).unwrap().to_string(), "\n");
//     //     assert_eq!(app.buffer.line(1).unwrap().to_string(), "h");
//     // }
// }

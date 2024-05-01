//
// #[test]
// fn test_text_under_cursor() {
//     let mut app = init();
//     app.move_right().unwrap();
//     assert_eq!(app.char_under_cursor().unwrap(), 'o');
//     app.move_right().unwrap();
//     app.move_right().unwrap();
//     app.move_right().unwrap();
//     assert_eq!(app.char_under_cursor().unwrap(), 't');
// }
//
// #[test]
// fn test_no_text_under_cursor() {
//     let mut app = init();
//     app.move_end_line().unwrap();
//     app.move_right().unwrap();
//     assert_eq!(app.char_under_cursor().unwrap(), '\n');
//     app.move_right().unwrap();
//     assert_eq!(app.char_under_cursor(), None);
//     app.move_down().unwrap();
//     assert_eq!(app.char_under_cursor(), None);
// }

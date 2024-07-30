mod helpers;
mod movements {
    mod move_down;
    mod move_end_of_file;
    mod move_end_of_line;
    mod move_left;
    mod move_next_long_word_end;
    mod move_next_long_word_start;
    mod move_next_word_end;
    mod move_next_word_start;
    mod move_prev_word_start;
    mod move_right;
    mod move_start_of_file;
    mod move_start_of_line;
    mod move_up;
}
mod mode_change {
    mod from_delete;
    mod from_goto;
    mod from_insert;
    mod from_normal;
}
mod text_delta {
    mod delete_lines;
    mod insert_text;
}

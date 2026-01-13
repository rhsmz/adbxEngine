pub mod constants;
pub mod line_column_calculation;
pub mod pixel_position_calculation;

pub use constants::{
    get_line_height, get_line_number_width, get_padding_left, get_padding_top, CHAR_WIDTH,
    LINE_HEIGHT, LINE_NUMBER_WIDTH, PADDING_LEFT, PADDING_TOP, TAB_WIDTH,
};
pub use line_column_calculation::{
    calculate_char_position_from_line_column, calculate_cursor_line_and_column, get_line_length,
};
pub use pixel_position_calculation::{
    calculate_cursor_position_from_mouse_click, calculate_cursor_position_from_mouse_click_default,
    calculate_cursor_x_pixel_position,
};

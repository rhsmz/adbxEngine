#[cfg(test)]
mod tests {
    use adbx_editor::ui::code_editor::cursor_position::*;

    #[test]
    fn test_calculate_cursor_line_and_column() {
        let text = "line1\nline2\nline3";
        let (line, col) = calculate_cursor_line_and_column(text, 0);
        assert_eq!(line, 0);
        assert_eq!(col, 0);

        let (line, col) = calculate_cursor_line_and_column(text, 5);
        assert_eq!(line, 0);
        assert_eq!(col, 5);

        let (line, col) = calculate_cursor_line_and_column(text, 6);
        assert_eq!(line, 1);
        assert_eq!(col, 0);

        let (line, col) = calculate_cursor_line_and_column(text, 12);
        assert_eq!(line, 1);
        assert_eq!(col, 6);
    }

    #[test]
    fn test_calculate_char_position_from_line_column() {
        let text = "line1\nline2\nline3";
        let pos = calculate_char_position_from_line_column(text, 0, 0);
        assert_eq!(pos, 0);

        let pos = calculate_char_position_from_line_column(text, 0, 5);
        assert_eq!(pos, 5);

        let pos = calculate_char_position_from_line_column(text, 1, 0);
        assert_eq!(pos, 6);

        let pos = calculate_char_position_from_line_column(text, 1, 5);
        assert_eq!(pos, 11);
    }

    #[test]
    fn test_calculate_cursor_x_pixel_position() {
        let content = "hello\nworld";
        let x = calculate_cursor_x_pixel_position(content, 0, 0);
        assert_eq!(x, 0.0);

        let x = calculate_cursor_x_pixel_position(content, 0, 5);
        assert_eq!(x, 5.0 * CHAR_WIDTH);

        let x = calculate_cursor_x_pixel_position(content, 1, 0);
        assert_eq!(x, 0.0);
    }

    #[test]
    fn test_get_line_length() {
        let text = "line1\nline2\nline3";
        assert_eq!(get_line_length(text, 0), 5);
        assert_eq!(get_line_length(text, 1), 5);
        assert_eq!(get_line_length(text, 2), 5);
        assert_eq!(get_line_length(text, 10), 0); // 存在しない行
    }

    #[test]
    fn test_constants() {
        assert_eq!(get_line_number_width(), LINE_NUMBER_WIDTH);
        assert_eq!(get_line_height(), LINE_HEIGHT);
        assert_eq!(get_padding_left(), PADDING_LEFT);
        assert_eq!(get_padding_top(), PADDING_TOP);
    }
}

/// カーソル位置から文字位置を計算
pub fn get_char_position(text: &str, line: usize, col: usize) -> usize {
    let mut current_line = 0;
    let mut current_col = 0;
    let mut pos = 0;

    for ch in text.chars() {
        if current_line == line && current_col == col {
            break;
        }
        if ch == '\n' {
            if current_line == line {
                break;
            }
            current_line += 1;
            current_col = 0;
        } else {
            current_col += 1;
        }
        pos += 1; // 文字数でカウント
    }

    pos
}

/// 指定行の長さを取得
pub fn get_line_length(text: &str, line_index: usize) -> usize {
    text.lines()
        .nth(line_index)
        .map(|line| line.chars().count())
        .unwrap_or(0)
}

/// カーソル位置を計算
pub fn get_cursor_position(text: &str, cursor_pos: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    let mut pos = 0;

    for ch in text.chars() {
        if pos >= cursor_pos {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        pos += 1;
    }

    (line, col)
}

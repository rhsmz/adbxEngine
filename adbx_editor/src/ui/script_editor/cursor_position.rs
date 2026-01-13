/// カーソル位置を計算（行と列を返す）
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
        pos += ch.len_utf8();
    }

    (line, col)
}

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
        pos += ch.len_utf8();
    }

    pos
}

/// 指定位置に文字を挿入
pub fn insert_char_at_position(text: &mut String, position: usize, ch: char) {
    text.insert(position, ch);
}

/// 指定位置の文字を削除
pub fn remove_char_at_position(text: &mut String, position: usize) {
    if position < text.len() {
        text.remove(position);
    }
}

/// 指定行の長さを取得
pub fn get_line_length(text: &str, line_index: usize) -> usize {
    text.lines().nth(line_index).map(|line| line.len()).unwrap_or(0)
}

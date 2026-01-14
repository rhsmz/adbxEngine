use super::constants::{
    CHAR_WIDTH, LINE_HEIGHT, LINE_NUMBER_WIDTH, PADDING_LEFT, PADDING_TOP, TAB_WIDTH,
};

/// カーソルのX位置を計算（文字幅を考慮）
pub fn calculate_cursor_x_pixel_position(content: &str, line: usize, col: usize) -> f32 {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(line_text) = lines.get(line) {
        // カーソル位置までの文字列を取得
        let text_before_cursor: String = line_text.chars().take(col).collect();
        // 簡易的な文字幅計算（等幅フォントを想定）
        text_before_cursor.len() as f32 * CHAR_WIDTH // 1文字あたり約6.6ピクセル（11ptフォント）
    } else {
        0.0
    }
}

/// 文字の表示幅を計算（タブ文字を考慮）
fn get_char_display_width(ch: char) -> f32 {
    match ch {
        '\t' => TAB_WIDTH,
        _ => CHAR_WIDTH,
    }
}

/// クリック位置からカーソル位置を計算
pub fn calculate_cursor_position_from_mouse_click(
    text: &str,
    click_x: f32,
    click_y: f32,
    line_number_width: f32,
    line_height: f32,
    padding_left: f32,
    padding_top: f32,
) -> usize {
    // クリック位置から行番号を計算
    let line_index = ((click_y - padding_top) / line_height).floor().max(0.0) as usize;

    // 行を取得
    let lines: Vec<&str> = text.lines().collect();
    if line_index >= lines.len() {
        // 最後の行の末尾
        return text.len();
    }

    let line = lines[line_index];

    // クリック位置から列を計算（行番号エリアを除く）
    let x_offset = click_x - line_number_width - padding_left;

    // 行の各文字位置を計算して、クリック位置に最も近い位置を見つける
    let mut current_x = 0.0;
    let mut best_pos = 0;
    let mut min_distance = f32::MAX;

    for (i, ch) in line.chars().enumerate() {
        let char_width = get_char_display_width(ch);
        let char_center = current_x + char_width / 2.0;

        let distance = (char_center - x_offset).abs();
        if distance < min_distance {
            min_distance = distance;
            best_pos = i;
        }

        current_x += char_width;

        // クリック位置を超えた場合、この位置を選択
        if current_x > x_offset {
            best_pos = i;
            break;
        }
    }

    // 行の先頭位置を計算
    let mut line_start = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == line_index {
            break;
        }
        line_start += line.len() + 1; // +1 for newline
    }

    // 行内の位置を文字位置に変換
    let mut char_pos = line_start;
    for (i, _) in line.chars().enumerate() {
        if i >= best_pos {
            break;
        }
        char_pos += 1;
    }

    char_pos
}

/// デフォルトパラメータでクリック位置からカーソル位置を計算
pub fn calculate_cursor_position_from_mouse_click_default(
    text: &str,
    click_x: f32,
    click_y: f32,
) -> usize {
    calculate_cursor_position_from_mouse_click(
        text,
        click_x,
        click_y,
        LINE_NUMBER_WIDTH,
        LINE_HEIGHT,
        PADDING_LEFT,
        PADDING_TOP,
    )
}

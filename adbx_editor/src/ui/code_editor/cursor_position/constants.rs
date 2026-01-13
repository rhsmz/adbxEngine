/// 文字幅の定数（等幅フォントを想定、ピクセル単位）
pub const CHAR_WIDTH: f32 = 6.6;
/// タブ幅の定数（スペース4つ分）
pub const TAB_WIDTH: f32 = CHAR_WIDTH * 4.0;
/// 行番号エリアの幅（ピクセル単位）
pub const LINE_NUMBER_WIDTH: f32 = 50.0;
/// 行の高さ（ピクセル単位）
pub const LINE_HEIGHT: f32 = 20.0;
/// パディング（ピクセル単位）
pub const PADDING_TOP: f32 = 10.0;
pub const PADDING_LEFT: f32 = 10.0;

/// カーソル位置計算用の定数を取得
pub fn get_line_number_width() -> f32 {
    LINE_NUMBER_WIDTH
}

pub fn get_line_height() -> f32 {
    LINE_HEIGHT
}

pub fn get_padding_left() -> f32 {
    PADDING_LEFT
}

pub fn get_padding_top() -> f32 {
    PADDING_TOP
}

use bevy::prelude::*;
use syntect::highlighting::Style;

/// syntectのStyleをBevyのColorに変換
pub fn convert_syntect_style_to_bevy_color(style: &Style) -> Color {
    // syntectのColorはr, g, b, aフィールドを持つ
    let r = style.foreground.r as f32 / 255.0;
    let g = style.foreground.g as f32 / 255.0;
    let b = style.foreground.b as f32 / 255.0;
    Color::srgb(r, g, b)
}

/// デフォルトのテキストカラー
pub fn get_default_text_color() -> Color {
    Color::srgb(0.9, 0.9, 0.9)
}

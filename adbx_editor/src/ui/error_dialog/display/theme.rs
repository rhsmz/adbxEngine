use super::super::ErrorType;
use bevy::prelude::*;

/// エラータイプに応じた色とアイコンを取得
pub fn get_error_theme(error_type: ErrorType) -> (Color, Color, &'static str) {
    match error_type {
        ErrorType::Error => (Color::srgb(0.3, 0.1, 0.1), Color::srgb(1.0, 0.3, 0.3), "❌"),
        ErrorType::Warning => (
            Color::srgb(0.3, 0.25, 0.1),
            Color::srgb(1.0, 0.8, 0.3),
            "⚠️",
        ),
        ErrorType::Info => (Color::srgb(0.1, 0.2, 0.3), Color::srgb(0.3, 0.6, 1.0), "ℹ️"),
    }
}

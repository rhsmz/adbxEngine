use super::super::resource::DockingSystem;
use bevy::prelude::*;

/// ドッキングUIの描画
pub fn draw_docking_ui(mut _commands: Commands, _docking: Res<DockingSystem>) {
    // ドッキングUIの描画処理
    // 現在はドラッグハンドラーで処理されているため、ここは空実装
}

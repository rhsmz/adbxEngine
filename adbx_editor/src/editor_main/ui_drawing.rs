use bevy::prelude::*;

/// エディタUIの描画
pub fn draw_editor_ui(
    _commands: Commands,
    _editor_app: Res<crate::editor_app::EditorApp>,
) {
    // メインエディタUIの描画
    // 基本的なレイアウトはbuild_editor_uiで構築済み
    // ここでは動的な更新を行う
}

use super::preview_generation::{detect_preview_type, update_preview};
use super::RealtimePreview;
use bevy::prelude::*;

/// コード変更を監視してプレビューを更新
pub fn update_preview_on_code_change(
    mut preview: ResMut<RealtimePreview>,
    code_editor: Res<crate::ui::code_editor::CodeEditor>,
) {
    if !preview.is_enabled {
        return;
    }

    // 現在のファイルの内容を取得
    if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab) {
        let preview_type = detect_preview_type(&active_file.path);
        update_preview(
            &mut preview,
            active_file.path.clone(),
            active_file.content.clone(),
            preview_type,
        );
    } else if let Some(ref current_file) = code_editor.current_file {
        let preview_type = detect_preview_type(current_file);
        update_preview(
            &mut preview,
            current_file.clone(),
            code_editor.content.clone(),
            preview_type,
        );
    }
}

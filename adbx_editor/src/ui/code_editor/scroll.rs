use crate::ui::code_editor::resource::CodeEditor;
use bevy::prelude::*;

/// コードエディタのスクロール処理
pub fn handle_code_editor_mouse_wheel_scroll(
    mut code_editor: ResMut<CodeEditor>,
    mut mouse_wheel_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseWheel>,
    _windows: Query<&Window>,
) {
    // エディタがフォーカスされている場合のみスクロールを処理
    if !code_editor.is_focused {
        return;
    }

    // 現在のファイルの内容を取得
    let content_text = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab)
    {
        &active_file.content
    } else if code_editor.content.is_empty() {
        "// コードエディタ\n// ファイルを開くか、新規ファイルを作成してください"
    } else {
        &code_editor.content
    };

    let total_lines = content_text.lines().count();
    let max_scroll = (total_lines as f32 - code_editor.visible_lines as f32).max(0.0);

    // マウスホイールイベントを処理
    for event in mouse_wheel_events.read() {
        let scroll_delta = match event.unit {
            bevy::input::mouse::MouseScrollUnit::Line => event.y * 3.0, // 3行ずつスクロール
            bevy::input::mouse::MouseScrollUnit::Pixel => event.y * 0.1, // ピクセル単位
        };

        // スクロール位置を更新
        code_editor.scroll_offset = (code_editor.scroll_offset - scroll_delta)
            .max(0.0)
            .min(max_scroll);

        // UIを再描画
        code_editor.content_entity = None;
    }
}

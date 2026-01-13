use bevy::prelude::*;

/// シーンビューエリア内でマウスカーソルがあるかを判定
/// カメラのビューポートを使用して判定（簡易実装）
pub fn is_cursor_in_scene_view_area(
    windows: &Query<&Window>,
    camera_query: &Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
) -> bool {
    let Ok(window) = windows.single() else {
        return false;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return false;
    };

    // カメラのビューポートを取得
    if let Ok((camera, _)) = camera_query.single() {
        // カメラのビューポートが設定されている場合、その範囲内かどうかをチェック
        if let Some(viewport_rect) = camera.physical_viewport_rect() {
            let window_height = window.resolution.height() as f32;
            let viewport_x = viewport_rect.min.x as f32;
            let viewport_y = window_height - (viewport_rect.max.y as f32); // Y軸を反転
            let viewport_width = viewport_rect.width() as f32;
            let viewport_height = viewport_rect.height() as f32;

            // マウス位置がビューポートの範囲内にあるかチェック
            return cursor_pos.x >= viewport_x
                && cursor_pos.x <= viewport_x + viewport_width
                && cursor_pos.y >= viewport_y
                && cursor_pos.y <= viewport_y + viewport_height;
        } else {
            // ビューポートが設定されていない場合、ウィンドウ全体をシーンビューとして扱う
            // （簡易実装）
            return true;
        }
    }

    false
}

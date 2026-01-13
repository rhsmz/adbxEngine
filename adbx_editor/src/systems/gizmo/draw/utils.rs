use super::super::resource::GizmoHandle;
use super::super::resource::GizmoInteraction;
use bevy::prelude::*;

/// カメラ距離に応じたGizmoスケールを計算
pub fn calculate_gizmo_scale(
    transform: &Transform,
    camera_query: &Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
) -> f32 {
    if let Ok((_, camera_transform)) = camera_query.single() {
        let camera_pos = camera_transform.translation();
        let distance = transform.translation.distance(camera_pos);

        // 距離に応じてスケールを調整（基準距離: 10.0、最小スケール: 0.5、最大スケール: 2.0）
        let base_distance = 10.0;
        let scale = (distance / base_distance).clamp(0.5, 2.0);
        scale
    } else {
        1.0 // カメラが見つからない場合はデフォルトスケール
    }
}

/// ハンドルの色を決定（ホバー時は明るく、アクティブ時はさらに明るく）
pub fn get_handle_color(
    gizmo_interaction: &GizmoInteraction,
    handle: GizmoHandle,
    base_r: f32,
    base_g: f32,
    base_b: f32,
) -> Color {
    if gizmo_interaction.active_handle == Some(handle) {
        // アクティブ時：黄色に近い明るい色
        Color::srgb(1.0, 1.0, 0.0)
    } else if gizmo_interaction.hovered_handle == Some(handle) {
        // ホバー時：明るくする
        Color::srgb(
            (base_r * 1.5).min(1.0),
            (base_g * 1.5).min(1.0),
            (base_b * 1.5).min(1.0),
        )
    } else {
        Color::srgb(base_r, base_g, base_b)
    }
}

/// 基本的な軸Gizmoの描画
pub fn draw_axis_gizmo(gizmos: &mut Gizmos, transform: &Transform) {
    // 位置を示すGizmo
    gizmos.sphere(transform.translation, 0.1, Color::srgb(1.0, 1.0, 1.0));

    // 軸を示す線
    let axis_length = 0.5;
    gizmos.line(
        transform.translation,
        transform.translation + transform.local_x() * axis_length,
        Color::srgb(1.0, 0.0, 0.0),
    );
    gizmos.line(
        transform.translation,
        transform.translation + transform.local_y() * axis_length,
        Color::srgb(0.0, 1.0, 0.0),
    );
    gizmos.line(
        transform.translation,
        transform.translation + transform.local_z() * axis_length,
        Color::srgb(0.0, 0.0, 1.0),
    );
}

use bevy::prelude::*;

/// 回転リングの描画
pub fn draw_rotation_ring(
    gizmos: &mut Gizmos,
    transform: &Transform,
    axis: Vec3,
    radius: f32,
    segments: usize,
    color: Color,
) {
    let axis = axis.normalize();
    let center = transform.translation;
    let up = if axis.dot(Vec3::Y).abs() > 0.9 {
        Vec3::X
    } else {
        Vec3::Y
    };
    let right = axis.cross(up).normalize();
    let forward = right.cross(axis).normalize();

    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

        let p1 = center + (right * angle1.cos() + forward * angle1.sin()) * radius;
        let p2 = center + (right * angle2.cos() + forward * angle2.sin()) * radius;

        gizmos.line(p1, p2, color);
    }
}

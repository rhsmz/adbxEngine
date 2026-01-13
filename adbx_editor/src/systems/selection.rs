use crate::communication;
use crate::ui::scene_view;
use bevy::prelude::*;

/// 選択システムのリソース
#[derive(Resource, Default)]
pub struct Selection {
    pub selected_entities: Vec<Entity>,
}

/// レイとAABB（軸平行バウンディングボックス）の交差判定
fn ray_aabb_intersection(
    ray_origin: Vec3,
    ray_direction: Vec3,
    aabb_min: Vec3,
    aabb_max: Vec3,
) -> Option<f32> {
    // ゼロ除算を防ぐ
    let epsilon = 1e-6;
    let inv_dir = Vec3::new(
        if ray_direction.x.abs() > epsilon {
            1.0 / ray_direction.x
        } else {
            f32::MAX
        },
        if ray_direction.y.abs() > epsilon {
            1.0 / ray_direction.y
        } else {
            f32::MAX
        },
        if ray_direction.z.abs() > epsilon {
            1.0 / ray_direction.z
        } else {
            f32::MAX
        },
    );

    let t1 = (aabb_min.x - ray_origin.x) * inv_dir.x;
    let t2 = (aabb_max.x - ray_origin.x) * inv_dir.x;
    let t3 = (aabb_min.y - ray_origin.y) * inv_dir.y;
    let t4 = (aabb_max.y - ray_origin.y) * inv_dir.y;
    let t5 = (aabb_min.z - ray_origin.z) * inv_dir.z;
    let t6 = (aabb_max.z - ray_origin.z) * inv_dir.z;

    let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
    let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

    if tmax < 0.0 || tmin > tmax {
        return None;
    }

    Some(if tmin < 0.0 { tmax } else { tmin })
}

/// TransformからAABBを推定（簡易実装）
/// 実際のメッシュデータにアクセスするには、Meshアセットが必要
fn get_aabb_from_transform(transform: &GlobalTransform) -> (Vec3, Vec3) {
    // Transformのスケールに基づいてAABBを推定
    let scale = transform.scale();
    let half_size = scale.max_element() * 0.5;
    let center = transform.translation();

    (
        center - Vec3::splat(half_size),
        center + Vec3::splat(half_size),
    )
}

/// レイとTransformベースのAABBの交差判定
fn ray_transform_intersection(
    ray_origin: Vec3,
    ray_direction: Vec3,
    transform: &GlobalTransform,
) -> Option<f32> {
    let (aabb_min, aabb_max) = get_aabb_from_transform(transform);
    ray_aabb_intersection(ray_origin, ray_direction, aabb_min, aabb_max)
}

/// Entityの選択処理（レイキャストを使用）
pub fn handle_selection(
    mut selection: ResMut<Selection>,
    communication: Res<communication::EditorRuntimeCommunication>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (
            With<bevy::camera::Camera3d>,
            Without<bevy::camera::Camera2d>,
        ),
    >,
    transform_query: Query<(Entity, &GlobalTransform), With<Transform>>,
) {
    // マウスクリックでEntityを選択
    if mouse_input.just_pressed(MouseButton::Left) {
        // UIとの干渉を防ぐため、シーンビューエリア内でのみ有効
        if !scene_view::is_cursor_in_scene_view_area(&windows, &camera_query) {
            return;
        }

        let Ok(window) = windows.single() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        // マウス位置を取得
        if let Some(cursor_position) = window.cursor_position() {
            // レイキャストを実行
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let mut closest_entity: Option<Entity> = None;
                let mut closest_distance = f32::MAX;

                // Transformを持つエンティティをチェック（AABB交差判定）
                for (entity, transform) in transform_query.iter() {
                    // AABBとの交差判定
                    if let Some(distance) =
                        ray_transform_intersection(ray.origin, ray.direction.normalize(), transform)
                    {
                        if distance < closest_distance && distance > 0.0 {
                            closest_distance = distance;
                            closest_entity = Some(entity);
                        }
                    } else {
                        // AABB交差がない場合、簡易的な距離チェック
                        let entity_position = transform.translation();
                        let distance = ray.origin.distance(entity_position);

                        // 簡易的な距離チェック（フォールバック）
                        if distance < closest_distance && distance < 5.0 {
                            closest_distance = distance;
                            closest_entity = Some(entity);
                        }
                    }
                }

                // 最も近いエンティティを選択
                let previous_selection = selection.selected_entities.clone();
                if let Some(entity) = closest_entity {
                    selection.selected_entities.clear();
                    selection.selected_entities.push(entity);
                } else {
                    // 何も選択されていない場合は選択をクリア
                    selection.selected_entities.clear();
                }

                // 選択が変更された場合、ランタイムに通知
                if selection.selected_entities != previous_selection {
                    communication::notify_entity_selection(
                        &*communication,
                        &selection.selected_entities,
                    );
                }
            }
        }
    }
}

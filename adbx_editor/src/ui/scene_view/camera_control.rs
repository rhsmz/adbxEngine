use bevy::prelude::*;
use super::resource::SceneView;

/// シーンビューのカメラ操作システム
pub fn handle_scene_view_input(
    mut scene_view: ResMut<SceneView>,
    mut camera_query: Query<&mut Transform, (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseMotion>,
    mut mouse_wheel_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseWheel>,
    windows: Query<&Window>,
    camera_query_for_viewport: Query<(&Camera, &GlobalTransform), (With<bevy::camera::Camera3d>, Without<bevy::camera::Camera2d>)>,
    time: Res<Time>,
) {
    // シーンビューエリア内でのみカメラ操作を有効にする
    // 簡易実装: カメラのビューポートを使用して判定
    let is_in_scene_view = if let Ok((camera, _)) = camera_query_for_viewport.single() {
        if let Some(viewport_rect) = camera.physical_viewport_rect() {
            if let Some(cursor_pos) = windows.single().ok().and_then(|w| w.cursor_position()) {
                let window_height = windows.single().unwrap().resolution.height() as f32;
                let viewport_x = viewport_rect.min.x as f32;
                let viewport_y = window_height - (viewport_rect.max.y as f32);
                let viewport_width = viewport_rect.width() as f32;
                let viewport_height = viewport_rect.height() as f32;
                
                cursor_pos.x >= viewport_x && cursor_pos.x <= viewport_x + viewport_width &&
                cursor_pos.y >= viewport_y && cursor_pos.y <= viewport_y + viewport_height
            } else {
                false
            }
        } else {
            // ビューポートが設定されていない場合、常に有効とする（簡易実装）
            true
        }
    } else {
        false
    };
    
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    
    // マウスホイールでズーム（シーンビューエリア内でのみ）
    if is_in_scene_view {
        for event in mouse_wheel_events.read() {
            let zoom_delta = match event.unit {
                bevy::input::mouse::MouseScrollUnit::Line => event.y * 0.5,
                bevy::input::mouse::MouseScrollUnit::Pixel => event.y * 0.01,
            };
            scene_view.orbit_distance = (scene_view.orbit_distance - zoom_delta)
                .clamp(1.0, 100.0);
        }
    }
    
    // キーボードでズーム（代替手段、常に有効）
    let zoom_speed = 5.0 * time.delta_secs();
    if keyboard_input.pressed(KeyCode::KeyQ) {
        scene_view.orbit_distance = (scene_view.orbit_distance - zoom_speed).max(1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        scene_view.orbit_distance = (scene_view.orbit_distance + zoom_speed).min(100.0);
    }
    
    // マウスドラッグでOrbit（シーンビューエリア内でのみ）
    if is_in_scene_view && mouse_input.pressed(MouseButton::Right) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }
        
        if delta.length() > 0.0 {
            // マウス感度
            let sensitivity = 0.005;
            scene_view.orbit_angles.x -= delta.x * sensitivity; // Yaw
            scene_view.orbit_angles.y = (scene_view.orbit_angles.y - delta.y * sensitivity)
                .clamp(0.1, std::f32::consts::PI - 0.1); // Pitch (上下の制限)
        }
    }
    
    // マウス中ボタンでPan（シーンビューエリア内でのみ）
    if is_in_scene_view && mouse_input.pressed(MouseButton::Middle) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }
        
        if delta.length() > 0.0 {
            // カメラの向きに基づいてPan方向を計算
            let yaw = scene_view.orbit_angles.x;
            
            // カメラの右方向と上方向を計算
            let right = Vec3::new(-yaw.sin(), 0.0, yaw.cos());
            let up = Vec3::Y;
            
            // Pan速度
            let pan_speed = scene_view.orbit_distance * 0.001;
            scene_view.orbit_target -= right * delta.x * pan_speed;
            scene_view.orbit_target += up * delta.y * pan_speed;
        }
    }
    
    // カメラ位置を更新
    let yaw = scene_view.orbit_angles.x;
    let pitch = scene_view.orbit_angles.y;
    
    let camera_position = scene_view.orbit_target + Vec3::new(
        scene_view.orbit_distance * pitch.sin() * yaw.sin(),
        scene_view.orbit_distance * pitch.cos(),
        scene_view.orbit_distance * pitch.sin() * yaw.cos(),
    );
    
    camera_transform.translation = camera_position;
    camera_transform.look_at(scene_view.orbit_target, Vec3::Y);
}

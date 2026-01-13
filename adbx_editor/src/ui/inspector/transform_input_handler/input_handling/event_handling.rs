use bevy::prelude::*;
use crate::ui::inspector::inspector_panel_resource::{InspectorInputState, TransformFieldType, InspectorPanel};
use super::super::field_access::get_transform_field_value;
use super::value_update::{update_value_with_wheel, update_value_with_drag, update_value_with_click};

/// マウスホイールイベントの処理
pub fn handle_mouse_wheel_event(
    transform_query: &mut Query<&mut Transform>,
    input_state: &InspectorInputState,
    wheel_event: &bevy::input::mouse::MouseWheel,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut operation_recorder: ResMut<crate::systems::operation_recording::OperationRecorder>,
) {
    if let Some((_entity, _field_type)) = input_state.editing_field {
        let sensitivity = if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            0.1
        } else {
            0.01
        };
        
        update_value_with_wheel(
            transform_query,
            input_state,
            wheel_event.y,
            sensitivity,
            operation_recorder,
        );
    }
}

/// ドラッグイベントの処理
pub fn handle_drag_event(
    transform_query: &mut Query<&mut Transform>,
    input_state: &mut ResMut<InspectorInputState>,
    current_pos: Vec2,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut operation_recorder: ResMut<crate::systems::operation_recording::OperationRecorder>,
) {
    if let Some(start_pos) = input_state.drag_start_mouse_pos {
        let delta_x = current_pos.x - start_pos.x;
        let sensitivity = if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            0.1
        } else {
            0.01
        };
        
        update_value_with_drag(
            transform_query,
            input_state,
            delta_x,
            sensitivity,
            operation_recorder,
        );
        
        input_state.drag_start_mouse_pos = Some(current_pos);
    }
}

/// フィールドクリックイベントの処理
pub fn handle_field_click(
    transform_query: &mut Query<&mut Transform>,
    input_state: &mut ResMut<InspectorInputState>,
    entity: Entity,
    field_type: TransformFieldType,
    cursor_pos: Vec2,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
) {
    input_state.editing_field = Some((entity, field_type));
    input_state.drag_start_mouse_pos = Some(cursor_pos);
    
    if let Ok(transform) = transform_query.get(entity) {
        input_state.drag_start_value = Some(get_transform_field_value(&transform, field_type));
    }
    
    // Ctrl+クリックで小さな増減
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
        let delta = if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            0.1
        } else {
            0.01
        };
        
        update_value_with_click(transform_query, entity, field_type, delta);
    }
}

/// ドラッグ終了イベントの処理
pub fn handle_drag_end(mut input_state: ResMut<InspectorInputState>) {
    input_state.editing_field = None;
    input_state.drag_start_value = None;
    input_state.drag_start_mouse_pos = None;
}

/// ボタンクリックイベントの処理（スクリプト削除など）
pub fn handle_button_click(
    mut inspector: ResMut<InspectorPanel>,
    name: &str,
    communication: ResMut<crate::communication::EditorRuntimeCommunication>,
) {
    if name.starts_with("DetachScriptButton_") {
        if let Some(entity_id_str) = name.strip_prefix("DetachScriptButton_") {
            if let Ok(entity_id) = entity_id_str.parse::<u32>() {
                if let Err(e) = crate::communication::send_to_runtime(
                    &*communication,
                    adbx_shared::EditorMessage::DetachScript { entity_id },
                ) {
                    bevy::log::error!("Failed to send detach script message: {}", e);
                } else {
                    bevy::log::info!("Detach script request sent for entity {}", entity_id);
                    inspector.content_entity = None;
                }
            }
        }
    }
}

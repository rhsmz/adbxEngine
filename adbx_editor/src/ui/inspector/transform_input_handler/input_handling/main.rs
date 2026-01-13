use super::event_handling::{
    handle_button_click, handle_drag_end, handle_drag_event, handle_field_click,
    handle_mouse_wheel_event,
};
use crate::systems::operation_recording::OperationRecorder;
use crate::ui::inspector::inspector_panel_resource::{InspectorInputState, InspectorPanel};
use bevy::prelude::*;

/// インスペクター入力の処理（改善版：マウスホイールとドラッグ対応）
pub fn handle_inspector_transform_input(
    mut inspector: ResMut<InspectorPanel>,
    mut input_state: ResMut<InspectorInputState>,
    mut transform_query: Query<&mut Transform>,
    mut operation_recorder: ResMut<OperationRecorder>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseWheel>,
    mut mouse_motion_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseMotion>,
    interaction_query: Query<
        (
            &Interaction,
            &crate::ui::inspector::inspector_panel_resource::TransformInputField,
        ),
        Changed<Interaction>,
    >,
    button_interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    communication: ResMut<crate::communication::EditorRuntimeCommunication>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
) {
    // ボタンクリック処理
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in button_interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                handle_button_click(&mut inspector, name.as_str(), &*communication);
            }
        }
    }

    // マウスホイールで値を増減
    for wheel_event in mouse_wheel_events.read() {
        handle_mouse_wheel_event(
            &mut transform_query,
            &input_state,
            &wheel_event,
            &keyboard_input,
            &mut *operation_recorder,
        );
    }

    // ドラッグで値を変更
    if let Some((_entity, _field_type)) = input_state.editing_field {
        if mouse_input.pressed(MouseButton::Left) {
            let _ = mouse_motion_events.read(); // イベントを消費
            if let Some(window) = windows.iter().next() {
                if let Some(current_pos) = window.cursor_position() {
                    handle_drag_event(
                        &mut transform_query,
                        &mut *input_state,
                        current_pos,
                        &keyboard_input,
                        &mut *operation_recorder,
                    );
                }
            }
        } else {
            // ドラッグ終了
            handle_drag_end(&mut *input_state);
        }
    }

    // フィールドをクリックして編集開始
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, field) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                if let Some(window) = windows.iter().next() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        handle_field_click(
                            &mut transform_query,
                            &mut input_state,
                            field.entity,
                            field.field_type,
                            cursor_pos,
                            &keyboard_input,
                        );
                    }
                }
            }
        }
    }
}

use bevy::prelude::*;
use crate::systems::operation_recording::{record_transform_changed, OperationRecorder};
use crate::ui::inspector::inspector_panel_resource::{InspectorInputState, TransformFieldType};
use super::super::field_access::{get_transform_field_value, update_transform_field_value};
use super::input_validation::normalize_transform_value;

/// マウスホイールで値を更新
pub fn update_value_with_wheel(
    transform_query: &mut Query<&mut Transform>,
    input_state: &InspectorInputState,
    wheel_delta: f32,
    sensitivity: f32,
    operation_recorder: &mut OperationRecorder,
) {
    if let Some((entity, field_type)) = input_state.editing_field {
        if let Ok(mut transform) = transform_query.get_mut(entity) {
            let old_transform = *transform;
            let change = wheel_delta * sensitivity;
            update_transform_field_value(&mut transform, field_type, change);
            
            // Transform変更を記録
            record_transform_changed(operation_recorder, entity, Some(old_transform), *transform);
        }
    }
}

/// ドラッグで値を更新
pub fn update_value_with_drag(
    transform_query: &mut Query<&mut Transform>,
    input_state: &mut InspectorInputState,
    delta_x: f32,
    sensitivity: f32,
    operation_recorder: &mut OperationRecorder,
) {
    if let Some((entity, field_type)) = input_state.editing_field {
        if let Ok(mut transform) = transform_query.get_mut(entity) {
            let old_transform = *transform;
            let change = delta_x * sensitivity;
            update_transform_field_value(&mut transform, field_type, change);
            
            // Transform変更を記録
            record_transform_changed(operation_recorder, entity, Some(old_transform), *transform);
        }
    }
}

/// クリックで値を更新（Ctrl+クリック）
pub fn update_value_with_click(
    transform_query: &mut Query<&mut Transform>,
    entity: Entity,
    field_type: TransformFieldType,
    delta: f32,
) {
    if let Ok(mut transform) = transform_query.get_mut(entity) {
        let normalized_delta = normalize_transform_value(field_type, delta);
        
        match field_type {
            TransformFieldType::TranslationX => {
                transform.translation.x += normalized_delta;
            }
            TransformFieldType::TranslationY => {
                transform.translation.y += normalized_delta;
            }
            TransformFieldType::TranslationZ => {
                transform.translation.z += normalized_delta;
            }
            TransformFieldType::RotationX => {
                let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
                let new_roll = (roll.to_degrees() + normalized_delta).to_radians();
                transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, new_roll, pitch, yaw);
            }
            TransformFieldType::RotationY => {
                let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
                let new_pitch = (pitch.to_degrees() + normalized_delta).to_radians();
                transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, roll, new_pitch, yaw);
            }
            TransformFieldType::RotationZ => {
                let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
                let new_yaw = (yaw.to_degrees() + normalized_delta).to_radians();
                transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, roll, pitch, new_yaw);
            }
            TransformFieldType::ScaleX => {
                transform.scale.x = normalize_transform_value(field_type, transform.scale.x + normalized_delta);
            }
            TransformFieldType::ScaleY => {
                transform.scale.y = normalize_transform_value(field_type, transform.scale.y + normalized_delta);
            }
            TransformFieldType::ScaleZ => {
                transform.scale.z = normalize_transform_value(field_type, transform.scale.z + normalized_delta);
            }
        }
    }
}

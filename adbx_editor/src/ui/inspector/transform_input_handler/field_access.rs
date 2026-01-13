use crate::ui::inspector::inspector_panel_resource::TransformFieldType;
use bevy::prelude::*;

/// Transformの値を取得
pub fn get_transform_field_value(transform: &Transform, field_type: TransformFieldType) -> f32 {
    match field_type {
        TransformFieldType::TranslationX => transform.translation.x,
        TransformFieldType::TranslationY => transform.translation.y,
        TransformFieldType::TranslationZ => transform.translation.z,
        TransformFieldType::RotationX => {
            let (roll, _, _) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            roll.to_degrees()
        }
        TransformFieldType::RotationY => {
            let (_, pitch, _) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            pitch.to_degrees()
        }
        TransformFieldType::RotationZ => {
            let (_, _, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            yaw.to_degrees()
        }
        TransformFieldType::ScaleX => transform.scale.x,
        TransformFieldType::ScaleY => transform.scale.y,
        TransformFieldType::ScaleZ => transform.scale.z,
    }
}

/// Transformの値を更新
pub fn update_transform_field_value(
    transform: &mut Transform,
    field_type: TransformFieldType,
    change: f32,
) {
    match field_type {
        TransformFieldType::TranslationX => transform.translation.x += change,
        TransformFieldType::TranslationY => transform.translation.y += change,
        TransformFieldType::TranslationZ => transform.translation.z += change,
        TransformFieldType::RotationX => {
            let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            let new_roll = (roll.to_degrees() + change).to_radians();
            transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, new_roll, pitch, yaw);
        }
        TransformFieldType::RotationY => {
            let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            let new_pitch = (pitch.to_degrees() + change).to_radians();
            transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, roll, new_pitch, yaw);
        }
        TransformFieldType::RotationZ => {
            let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
            let new_yaw = (yaw.to_degrees() + change).to_radians();
            transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, roll, pitch, new_yaw);
        }
        TransformFieldType::ScaleX => transform.scale.x = (transform.scale.x + change).max(0.01),
        TransformFieldType::ScaleY => transform.scale.y = (transform.scale.y + change).max(0.01),
        TransformFieldType::ScaleZ => transform.scale.z = (transform.scale.z + change).max(0.01),
    }
}

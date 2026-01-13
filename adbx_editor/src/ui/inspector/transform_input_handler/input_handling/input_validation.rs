use bevy::prelude::*;
use crate::ui::inspector::inspector_panel_resource::TransformFieldType;

/// 入力値の検証
pub fn validate_transform_value(field_type: TransformFieldType, value: f32) -> bool {
    match field_type {
        TransformFieldType::ScaleX | TransformFieldType::ScaleY | TransformFieldType::ScaleZ => {
            value > 0.0
        }
        _ => true,
    }
}

/// 入力値の正規化
pub fn normalize_transform_value(field_type: TransformFieldType, value: f32) -> f32 {
    match field_type {
        TransformFieldType::ScaleX | TransformFieldType::ScaleY | TransformFieldType::ScaleZ => {
            value.max(0.01)
        }
        _ => value,
    }
}

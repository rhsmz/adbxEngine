use bevy::prelude::*;
use std::collections::HashMap;

/// インスペクターパネルのリソース
#[derive(Resource, Default)]
pub struct InspectorPanel {
    pub selected_entity: Option<Entity>,
    pub content_entity: Option<Entity>, // 現在のコンテンツエンティティ
    pub transform_input_values: HashMap<Entity, TransformInputValues>, // エンティティごとの入力値
    pub last_update_time: f64, // 最後の更新時刻（遅延更新用）
    pub update_throttle: f64, // 更新間隔（秒単位、デフォルト0.1秒）
    pub pending_update: bool, // 更新待ちフラグ
}

/// Transform入力値の一時保存
#[derive(Debug, Clone)]
pub struct TransformInputValues {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for TransformInputValues {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

/// Transform入力フィールドのマーカーコンポーネント
#[derive(Component)]
pub struct TransformInputField {
    pub entity: Entity,
    pub field_type: TransformFieldType,
}

/// 入力フィールドの編集状態を管理
#[derive(Resource, Default)]
pub struct InspectorInputState {
    pub editing_field: Option<(Entity, TransformFieldType)>,
    pub drag_start_value: Option<f32>,
    pub drag_start_mouse_pos: Option<Vec2>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformFieldType {
    TranslationX,
    TranslationY,
    TranslationZ,
    RotationX,
    RotationY,
    RotationZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

/// インスペクターコンテンツのマーカーコンポーネント
#[derive(Component)]
pub struct InspectorContent;

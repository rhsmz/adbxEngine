use bevy::prelude::*;
use bevy::math::{Vec3, Quat};
use std::collections::HashMap;

/// TransformコンポーネントへのアクセスをLuaスクリプトに提供するブリッジ
/// Transformの変更をキューに保存し、システムで適用します
#[derive(Resource, Default)]
pub struct TransformBridge {
    pub transform_updates: HashMap<u32, TransformUpdate>,
}

#[derive(Debug, Clone)]
pub enum TransformUpdate {
    SetTranslation { x: f32, y: f32, z: f32 },
    SetRotation { x: f32, y: f32, z: f32, w: f32 },
    SetScale { x: f32, y: f32, z: f32 },
    Translate { x: f32, y: f32, z: f32 },
}

/// TransformBridgeから更新を適用するシステム
pub fn apply_transform_updates(
    mut bridge: ResMut<TransformBridge>,
    mut transform_query: Query<(Entity, &mut Transform)>,
) {
    let updates: Vec<_> = bridge.transform_updates.drain().collect();

    for (entity_id, update) in updates {
        // Entity IDからEntityを検索
        for (entity, mut transform) in transform_query.iter_mut() {
            if entity.index() == entity_id {
                match update {
                    TransformUpdate::SetTranslation { x, y, z } => {
                        transform.translation = Vec3::new(x, y, z);
                    }
                    TransformUpdate::SetRotation { x, y, z, w } => {
                        transform.rotation = Quat::from_xyzw(x, y, z, w);
                    }
                    TransformUpdate::SetScale { x, y, z } => {
                        transform.scale = Vec3::new(x, y, z);
                    }
                    TransformUpdate::Translate { x, y, z } => {
                        transform.translation += Vec3::new(x, y, z);
                    }
                }
                break;
            }
        }
    }
}

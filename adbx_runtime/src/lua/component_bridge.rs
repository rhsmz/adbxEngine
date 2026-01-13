use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component操作をLuaスクリプトに提供するブリッジ
/// Componentの取得と設定をキューに保存し、システムで処理します
#[derive(Resource, Default)]
pub struct ComponentBridge {
    pub component_get_requests: Vec<ComponentRequest>, // Component取得リクエスト
    pub component_set_requests: Vec<ComponentUpdate>,  // Component設定リクエスト
    pub component_values: HashMap<String, serde_json::Value>, // 取得したComponent値のキャッシュ（Entity ID + Component名 -> 値）
}

/// Component取得リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRequest {
    pub entity_id: u32,
    pub component_name: String,
}

/// Component更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentUpdate {
    pub entity_id: u32,
    pub component_name: String,
    pub component_data: serde_json::Value,
}

/// ComponentBridgeからComponentを取得・設定するシステム
/// よく使われるComponent（Sprite、Mesh、Materialなど）を個別に処理
pub fn process_component_operations(
    mut bridge: ResMut<ComponentBridge>,
    sprite_query: Query<(Entity, &Sprite), With<Sprite>>,
) {
    // Component取得リクエストを処理
    let get_requests: Vec<ComponentRequest> = bridge.component_get_requests.drain(..).collect();
    for request in get_requests {
        let cache_key = format!("{}_{}", request.entity_id, request.component_name);

        match request.component_name.as_str() {
            "Sprite" => {
                for (entity, sprite) in sprite_query.iter() {
                    if entity.index() == request.entity_id {
                        let color_srgba = sprite.color.to_srgba();
                        let sprite_value = serde_json::json!({
                            "color": {
                                "r": color_srgba.red,
                                "g": color_srgba.green,
                                "b": color_srgba.blue,
                                "a": color_srgba.alpha,
                            },
                            "custom_size": sprite.custom_size.map(|size| {
                                serde_json::json!({
                                    "x": size.x,
                                    "y": size.y,
                                })
                            }),
                            "flip_x": sprite.flip_x,
                            "flip_y": sprite.flip_y,
                        });
                        bridge.component_values.insert(cache_key, sprite_value);
                        break;
                    }
                }
            }
            "Mesh" | "Material" => {
                // Handle<Mesh>とHandle<StandardMaterial>はComponentではないため、
                // 直接Queryで取得できない。必要に応じて別の方法で実装する。
                bevy::log::warn!("Mesh and Material components are not directly queryable. This feature needs to be implemented differently.");
            }
            _ => {
                bevy::log::warn!("Unknown component requested: {}", request.component_name);
            }
        }
    }

    // Component設定リクエストを処理
    // 注意: 実際のComponentの変更は、個別のシステムで処理する必要があります
    // ここではログのみ出力
    for update in bridge.component_set_requests.iter() {
        bevy::log::debug!(
            "Component set request: Entity {} Component {} = {:?}",
            update.entity_id,
            update.component_name,
            update.component_data
        );
    }
    bridge.component_set_requests.clear();
}

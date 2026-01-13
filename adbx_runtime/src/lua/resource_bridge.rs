use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource操作をLuaスクリプトに提供するブリッジ
/// Resourceの変更をキューに保存し、システムで適用します
#[derive(Resource, Default)]
pub struct ResourceBridge {
    pub resource_get_requests: Vec<String>, // Resource名のリスト
    pub resource_set_requests: HashMap<String, serde_json::Value>, // Resource名と値のマップ
    pub resource_values: HashMap<String, serde_json::Value>, // 取得したResource値のキャッシュ
}

/// Resource操作の種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceOperation {
    Get {
        resource_name: String,
    },
    Set {
        resource_name: String,
        value: serde_json::Value,
    },
}

/// ResourceBridgeからResourceを取得するシステム
/// よく使われるResource（Time、Window、Inputなど）を個別に処理
pub fn process_resource_operations(
    mut bridge: ResMut<ResourceBridge>,
    time: Res<Time>,
    windows: Query<&Window>,
    keyboard_input: Res<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>,
    mouse_input: Res<bevy::input::ButtonInput<bevy::input::mouse::MouseButton>>,
) {
    // Resource取得リクエストを処理
    let get_requests: Vec<String> = bridge.resource_get_requests.drain(..).collect();
    for resource_name in get_requests {
        match resource_name.as_str() {
            "Time" => {
                let time_value = serde_json::json!({
                    "delta_seconds": time.delta_secs(),
                    "elapsed_seconds": time.elapsed_secs(),
                    "delta": time.delta().as_secs_f64(),
                    "elapsed": time.elapsed().as_secs_f64(),
                });
                bridge
                    .resource_values
                    .insert("Time".to_string(), time_value);
            }
            "Window" => {
                if let Some(window) = windows.iter().next() {
                    let window_value = serde_json::json!({
                        "width": window.width(),
                        "height": window.height(),
                        "title": window.title.clone(),
                        "mode": format!("{:?}", window.mode),
                        "resizable": window.resizable,
                        "decorations": window.decorations,
                        "transparent": window.transparent,
                    });
                    bridge
                        .resource_values
                        .insert("Window".to_string(), window_value);
                }
            }
            "KeyboardInput" => {
                // 押されているキーのリストを取得
                let pressed_keys: Vec<String> = keyboard_input
                    .get_pressed()
                    .map(|key| format!("{:?}", key))
                    .collect();
                let keyboard_value = serde_json::json!({
                    "pressed_keys": pressed_keys,
                });
                bridge
                    .resource_values
                    .insert("KeyboardInput".to_string(), keyboard_value);
            }
            "MouseInput" => {
                // 押されているマウスボタンのリストを取得
                let pressed_buttons: Vec<String> = mouse_input
                    .get_pressed()
                    .map(|button| format!("{:?}", button))
                    .collect();
                let mouse_value = serde_json::json!({
                    "pressed_buttons": pressed_buttons,
                });
                bridge
                    .resource_values
                    .insert("MouseInput".to_string(), mouse_value);
            }
            "CursorPosition" => {
                // Windowからカーソル位置を取得
                if let Some(window) = windows.iter().next() {
                    let cursor_value = serde_json::json!({
                        "position": window.cursor_position().map(|pos| serde_json::json!({
                            "x": pos.x,
                            "y": pos.y,
                        })),
                    });
                    bridge
                        .resource_values
                        .insert("CursorPosition".to_string(), cursor_value);
                }
            }
            _ => {
                bevy::log::warn!("Unknown resource requested: {}", resource_name);
            }
        }
    }

    // Resource設定リクエストを処理
    // 注意: 実際のResourceの変更は、個別のシステムで処理する必要があります
    // ここではログのみ出力
    for (resource_name, _value) in bridge.resource_set_requests.iter() {
        bevy::log::debug!("Resource set request: {} = {:?}", resource_name, _value);
    }
    bridge.resource_set_requests.clear();
}

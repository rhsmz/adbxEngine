use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Event操作をLuaスクリプトに提供するブリッジ
/// Eventの送信と受信をキューに保存し、システムで処理します
#[derive(Resource, Default)]
pub struct EventBridge {
    pub event_send_queue: Vec<EventMessage>, // 送信するEventのキュー
    pub event_receive_buffer: HashMap<String, Vec<serde_json::Value>>, // 受信したEventのバッファ（Event名 -> 値のリスト）
}

/// Eventメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event_name: String,
    pub event_data: serde_json::Value,
}

/// カスタムEvent型（Luaスクリプトから送信可能な汎用Event）
#[derive(Debug, Clone, Serialize, Deserialize, bevy::prelude::Message)]
pub struct CustomEvent {
    pub name: String,
    pub data: serde_json::Value,
}

impl CustomEvent {
    pub fn new(name: String, data: serde_json::Value) -> Self {
        Self { name, data }
    }
}

/// EventBridgeからEventを送信するシステム
/// CustomEventとして送信します
pub fn process_event_operations(
    mut bridge: ResMut<EventBridge>,
    mut custom_event_writer: bevy::prelude::MessageWriter<CustomEvent>,
) {
    // Event送信キューを処理
    for event_msg in bridge.event_send_queue.drain(..) {
        let custom_event =
            CustomEvent::new(event_msg.event_name.clone(), event_msg.event_data.clone());
        custom_event_writer.write(custom_event);
        bevy::log::debug!(
            "Event sent: {} = {:?}",
            event_msg.event_name,
            event_msg.event_data
        );
    }
}

/// EventBridgeにEventを受信するシステム
/// CustomEventを受信してバッファに追加します
pub fn receive_events(
    mut bridge: ResMut<EventBridge>,
    mut custom_event_reader: bevy::prelude::MessageReader<CustomEvent>,
) {
    // CustomEventを受信してバッファに追加
    for event in custom_event_reader.read() {
        let event_name = event.name.clone();
        let event_data = event.data.clone();

        // バッファに追加
        bridge
            .event_receive_buffer
            .entry(event_name)
            .or_insert_with(Vec::new)
            .push(event_data);
    }
}

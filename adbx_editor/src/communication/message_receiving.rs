use super::EditorRuntimeCommunication;
use adbx_shared::RuntimeMessage;
use bevy::prelude::*;

/// ランタイムからメッセージを受信
pub fn receive_from_runtime(communication: &mut EditorRuntimeCommunication) -> Vec<RuntimeMessage> {
    let mut messages = Vec::new();

    if communication.is_separated {
        // 分離実行モード：TCP通信を使用
        if let Some(ref stream) = communication.tcp_stream {
            let mut stream = stream.lock().unwrap();
            // ノンブロッキング読み取り
            stream.set_nonblocking(true).ok();

            // メッセージ長を読み取り（4バイト）
            let mut len_bytes = [0u8; 4];
            match stream.read_exact(&mut len_bytes) {
                Ok(_) => {
                    let len = u32::from_le_bytes(len_bytes) as usize;

                    // メッセージ本体を読み取り
                    let mut message_bytes = vec![0u8; len];
                    if stream.read_exact(&mut message_bytes).is_ok() {
                        if let Ok(message_str) = String::from_utf8(message_bytes) {
                            if let Ok(message) = serde_json::from_str::<RuntimeMessage>(&message_str) {
                                messages.push(message);
                            }
                        }
                    }
                }
                Err(e) => {
                    // WouldBlockは正常（データなし）、それ以外のエラーはログ出力
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        bevy::log::warn!("TCP receive failed: {}", e);
                    }
                }
            }
        }
    } else {
        // 同一プロセスモード：mpscチャネルを使用
        if let Some(ref rx) = communication.runtime_rx {
            let rx = rx.lock().unwrap();
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }
    }

    messages
}

use std::io::Read;

/// ランタイムからのメッセージを処理するシステム
pub fn handle_runtime_messages(mut communication: ResMut<EditorRuntimeCommunication>) {
    let messages = receive_from_runtime(communication.as_mut());

    for message in messages {
        match message {
            RuntimeMessage::SceneLoaded { scene } => {
                bevy::log::info!("Scene loaded in runtime: {}", scene.name);
            }
            RuntimeMessage::EntityUpdated { entity_id } => {
                bevy::log::debug!("Entity updated in runtime: {}", entity_id);
            }
            RuntimeMessage::Error { message } => {
                bevy::log::error!("Runtime error: {}", message);
            }
            RuntimeMessage::HotReloaded { asset_path } => {
                bevy::log::info!("Asset hot reloaded: {}", asset_path);
            }
            RuntimeMessage::ScriptExecuted {
                success,
                error_message,
            } => {
                if success {
                    bevy::log::info!("Script executed successfully");
                } else {
                    let error_msg = error_message.as_deref().unwrap_or("Unknown error");
                    bevy::log::error!("Script execution failed: {}", error_msg);
                }
            }
            RuntimeMessage::ScriptAttached {
                entity_id,
                script_path,
            } => {
                bevy::log::info!("Script attached to entity {}: {}", entity_id, script_path);
            }
            RuntimeMessage::ScriptDetached { entity_id } => {
                bevy::log::info!("Script detached from entity {}", entity_id);
            }
        }
    }
}

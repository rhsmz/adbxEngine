use crate::lua::vm::LuaVm;
use adbx_shared::{EditorMessage, RuntimeMessage};
use bevy::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

/// ランタイム-エディタ通信リソース
#[derive(Resource)]
pub struct RuntimeEditorCommunication {
    pub editor_rx: Option<Arc<Mutex<mpsc::Receiver<EditorMessage>>>>,
    pub runtime_tx: Option<Arc<Mutex<mpsc::Sender<RuntimeMessage>>>>,
    pub tcp_stream: Option<Arc<Mutex<TcpStream>>>,
    pub is_separated: bool,
}

impl Default for RuntimeEditorCommunication {
    fn default() -> Self {
        Self {
            editor_rx: None,
            runtime_tx: None,
            tcp_stream: None,
            is_separated: false,
        }
    }
}

/// エディタからメッセージを受信
pub fn receive_from_editor(communication: &mut RuntimeEditorCommunication) -> Vec<EditorMessage> {
    let mut messages = Vec::new();

    if communication.is_separated {
        // 分離実行モード：TCP通信を使用
        if let Some(ref stream) = communication.tcp_stream {
            let mut stream = stream.lock().unwrap();
            stream.set_nonblocking(true).ok();

            // メッセージ長を読み取り（4バイト）
            let mut len_bytes = [0u8; 4];
            if stream.read_exact(&mut len_bytes).is_ok() {
                let len = u32::from_le_bytes(len_bytes) as usize;

                // メッセージ本体を読み取り
                let mut message_bytes = vec![0u8; len];
                if stream.read_exact(&mut message_bytes).is_ok() {
                    if let Ok(message_str) = String::from_utf8(message_bytes) {
                        if let Ok(message) = serde_json::from_str::<EditorMessage>(&message_str) {
                            messages.push(message);
                        }
                    }
                }
            }
        }
    } else {
        // 同一プロセスモード：mpscチャネルを使用
        if let Some(ref rx) = communication.editor_rx {
            let rx = rx.lock().unwrap();
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }
    }

    messages
}

/// エディタにメッセージを送信
pub fn send_to_editor(
    communication: &RuntimeEditorCommunication,
    message: RuntimeMessage,
) -> Result<(), String> {
    if communication.is_separated {
        // 分離実行モード：TCP通信を使用
        if let Some(ref stream) = communication.tcp_stream {
            let mut stream = stream.lock().unwrap();
            let message_json = serde_json::to_string(&message)
                .map_err(|e| format!("Failed to serialize message: {}", e))?;

            // メッセージ長を送信（4バイト）
            let len = message_json.len() as u32;
            stream
                .write_all(&len.to_le_bytes())
                .map_err(|e| format!("Failed to write message length: {}", e))?;

            // メッセージ本体を送信
            stream
                .write_all(message_json.as_bytes())
                .map_err(|e| format!("Failed to write message: {}", e))?;

            stream
                .flush()
                .map_err(|e| format!("Failed to flush stream: {}", e))?;

            Ok(())
        } else {
            Err("TCP stream not connected".to_string())
        }
    } else {
        // 同一プロセスモード：mpscチャネルを使用
        if let Some(ref tx) = communication.runtime_tx {
            let tx = tx.lock().unwrap();
            tx.send(message)
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err("Communication channel not initialized".to_string())
        }
    }
}

/// エディタからのメッセージを処理するシステム
pub fn handle_editor_messages(
    mut commands: Commands,
    mut communication: ResMut<RuntimeEditorCommunication>,
    lua_vm: Res<LuaVm>,
) {
    let messages = receive_from_editor(communication.as_mut());

    for message in messages {
        match message {
            EditorMessage::ExecuteScript { script_content } => {
                // 一時的なEntityを作成してスクリプトを実行
                let temp_entity = commands.spawn_empty().id();

                // Lua VMでスクリプトを実行
                let lua = lua_vm.lua.lock().unwrap();
                let result = lua.scope(|_scope| {
                    let globals = lua.globals();
                    globals.set("entity_id", temp_entity.index())?;
                    lua.load(&script_content).exec()?;
                    Ok::<(), mlua::Error>(())
                });

                match result {
                    Ok(_) => {
                        // 実行成功
                        if let Err(e) = send_to_editor(
                            &communication,
                            RuntimeMessage::ScriptExecuted {
                                success: true,
                                error_message: None,
                            },
                        ) {
                            bevy::log::error!("Failed to send script execution result: {}", e);
                        }
                        bevy::log::info!("Script executed successfully");
                    }
                    Err(e) => {
                        // 実行失敗
                        let error_msg = format!("Lua error: {}", e);
                        if let Err(send_err) = send_to_editor(
                            &communication,
                            RuntimeMessage::ScriptExecuted {
                                success: false,
                                error_message: Some(error_msg.clone()),
                            },
                        ) {
                            bevy::log::error!(
                                "Failed to send script execution result: {}",
                                send_err
                            );
                        }
                        bevy::log::error!("Script execution failed: {}", error_msg);
                    }
                }

                // 一時的なEntityを削除
                commands.entity(temp_entity).despawn();
            }
            EditorMessage::AttachScript {
                entity_id,
                script_path,
            } => {
                use crate::lua::component::{LuaScript, LuaScriptState};
                use bevy::ecs::entity::Entity;

                // Entity IDからEntityを取得（Bevy 0.17では、Entityは内部的にu32のインデックスを使用）
                let entity = Entity::from_bits(entity_id as u64);

                // スクリプトをアタッチ
                let script_path_buf = std::path::PathBuf::from(&script_path);
                if script_path_buf.exists() {
                    commands
                        .entity(entity)
                        .insert(LuaScript::from_path(script_path_buf.clone()));
                    commands.entity(entity).insert(LuaScriptState::Loaded);

                    if let Err(e) = send_to_editor(
                        &communication,
                        RuntimeMessage::ScriptAttached {
                            entity_id,
                            script_path: script_path.clone(),
                        },
                    ) {
                        bevy::log::error!("Failed to send script attach result: {}", e);
                    }
                    bevy::log::info!("Script attached to entity {}: {}", entity_id, script_path);
                } else {
                    let error_msg = format!("Script file not found: {}", script_path);
                    if let Err(e) = send_to_editor(
                        &communication,
                        RuntimeMessage::Error {
                            message: error_msg.clone(),
                        },
                    ) {
                        bevy::log::error!("Failed to send error message: {}", e);
                    }
                    bevy::log::error!("Failed to attach script: {}", error_msg);
                }
            }
            EditorMessage::DetachScript { entity_id } => {
                use crate::lua::component::{LuaScript, LuaScriptState};
                use bevy::ecs::entity::Entity;

                // Entity IDからEntityを取得（Bevy 0.17では、Entityは内部的にu32のインデックスを使用）
                let entity = Entity::from_bits(entity_id as u64);

                // スクリプトを削除
                commands.entity(entity).remove::<LuaScript>();
                commands.entity(entity).remove::<LuaScriptState>();

                if let Err(e) =
                    send_to_editor(&communication, RuntimeMessage::ScriptDetached { entity_id })
                {
                    bevy::log::error!("Failed to send script detach result: {}", e);
                }
                bevy::log::info!("Script detached from entity {}", entity_id);
            }
            _ => {
                // 他のメッセージタイプは未実装
                bevy::log::debug!("Unhandled editor message: {:?}", message);
            }
        }
    }
}

use super::EditorRuntimeCommunication;
use adbx_shared::EditorMessage;

/// エディタからランタイムにメッセージを送信
pub fn send_to_runtime(
    communication: &EditorRuntimeCommunication,
    message: EditorMessage,
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
        if let Some(ref tx) = communication.editor_tx {
            let tx = tx.lock().unwrap();
            tx.send(message)
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err("Communication channel not initialized".to_string())
        }
    }
}

use std::io::Write;

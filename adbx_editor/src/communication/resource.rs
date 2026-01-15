use adbx_shared::{EditorMessage, RuntimeMessage};
use bevy::prelude::*;
use std::net::TcpStream;
use std::process::Child;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// 通信接続状態
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// エディタ-ランタイム通信リソース
#[derive(Resource)]
pub struct EditorRuntimeCommunication {
    pub editor_tx: Option<Arc<Mutex<mpsc::Sender<EditorMessage>>>>,
    pub runtime_rx: Option<Arc<Mutex<mpsc::Receiver<RuntimeMessage>>>>,
    pub runtime_process: Option<Child>,
    pub tcp_stream: Option<Arc<Mutex<TcpStream>>>,
    pub is_separated: bool,    // 分離実行モードかどうか
    pub tcp_port: Option<u16>, // TCP通信のポート番号
    pub connection_state: ConnectionState, // 接続状態
    pub last_connection_attempt: Option<Instant>, // 最後の接続試行時刻
    pub next_retry_at: Option<Instant>, // 次回再試行時刻
    pub connection_attempts: u32, // 接続試行回数
}

impl Default for EditorRuntimeCommunication {
    fn default() -> Self {
        Self {
            editor_tx: None,
            runtime_rx: None,
            runtime_process: None,
            tcp_stream: None,
            is_separated: false,
            tcp_port: None,
            connection_state: ConnectionState::Disconnected,
            last_connection_attempt: None,
            next_retry_at: None,
            connection_attempts: 0,
        }
    }
}

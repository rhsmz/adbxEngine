use bevy::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::process::Child;
use std::net::TcpStream;
use adbx_shared::{EditorMessage, RuntimeMessage};

/// エディタ-ランタイム通信リソース
#[derive(Resource)]
pub struct EditorRuntimeCommunication {
    pub editor_tx: Option<Arc<Mutex<mpsc::Sender<EditorMessage>>>>,
    pub runtime_rx: Option<Arc<Mutex<mpsc::Receiver<RuntimeMessage>>>>,
    pub runtime_process: Option<Child>,
    pub tcp_stream: Option<Arc<Mutex<TcpStream>>>,
    pub is_separated: bool, // 分離実行モードかどうか
    pub tcp_port: Option<u16>, // TCP通信のポート番号
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
        }
    }
}

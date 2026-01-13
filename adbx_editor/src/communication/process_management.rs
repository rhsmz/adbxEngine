use std::process::{Child, Command};
use std::net::TcpListener;
use std::thread;
use super::EditorRuntimeCommunication;

/// ランタイムプロセスを起動（分離実行モード）
pub fn start_runtime_process(
    communication: &mut EditorRuntimeCommunication,
    project_path: Option<&std::path::Path>,
) -> Result<(), String> {
    // TCPサーバーを起動（エディタ側）
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind TCP listener: {}", e))?;
    
    let port = listener.local_addr()
        .map_err(|e| format!("Failed to get local address: {}", e))?
        .port();
    
    communication.tcp_port = Some(port);
    
    // ランタイムプロセスを起動
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
       .arg("--bin")
       .arg("adbx_runtime")
       .arg("--")
       .arg("--port")
       .arg(port.to_string());
    
    if let Some(project_path) = project_path {
        cmd.arg("--project")
           .arg(project_path.to_string_lossy().as_ref());
    }
    
    let child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn runtime process: {}", e))?;
    
    communication.runtime_process = Some(child);
    communication.is_separated = true;
    
    // TCP接続を待機
    thread::spawn(move || {
        match listener.accept() {
            Ok((_stream, _)) => {
                bevy::log::info!("Runtime process connected via TCP");
                // ストリームは別途管理する必要がある
                // ここでは簡易実装として、グローバルなストレージを使用
            }
            Err(e) => {
                bevy::log::error!("Failed to accept TCP connection: {}", e);
            }
        }
    });
    
    Ok(())
}

/// ランタイムプロセスを停止
pub fn stop_runtime_process(
    communication: &mut EditorRuntimeCommunication,
) -> Result<(), String> {
    if let Some(mut process) = communication.runtime_process.take() {
        process.kill()
            .map_err(|e| format!("Failed to kill runtime process: {}", e))?;
        communication.is_separated = false;
    }
    Ok(())
}

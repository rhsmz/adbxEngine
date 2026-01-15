use super::EditorRuntimeCommunication;
use std::net::TcpListener;
use std::process::Command;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// ランタイムプロセスを起動（分離実行モード）
pub fn start_runtime_process(
    communication: &mut EditorRuntimeCommunication,
    project_path: Option<&std::path::Path>,
) -> Result<(), String> {
    // 接続状態をConnectingに設定
    communication.connection_state = super::resource::ConnectionState::Connecting;
    communication.last_connection_attempt = Some(std::time::Instant::now());
    communication.connection_attempts += 1;

    // TCPサーバーを起動（エディタ側）
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind TCP listener: {}", e))?;

    let port = listener
        .local_addr()
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

    let child = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn runtime process: {}", e))?;

    communication.runtime_process = Some(child);
    communication.is_separated = true;

    // TCP接続を待機し、接続されたストリームをチャネル経由で受け渡す
    let (tx, rx) = mpsc::channel::<std::net::TcpStream>();
    communication.tcp_stream = None; // 念のためクリア

    thread::spawn(move || {
        match listener.accept() {
            Ok((stream, _)) => {
                bevy::log::info!("Runtime process connected via TCP");
                if let Err(_) = tx.send(stream) {
                    bevy::log::error!("Failed to send TCP stream to main thread");
                }
            }
            Err(e) => {
                bevy::log::error!("Failed to accept TCP connection: {}", e);
            }
        }
    });

    // 接続を待機してtcp_streamに設定
    // タイムアウト付きで待機（5秒）
    match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(stream) => {
            communication.tcp_stream = Some(Arc::new(Mutex::new(stream)));
            communication.connection_state = super::resource::ConnectionState::Connected;
            bevy::log::info!("TCP connection established successfully");
            Ok(())
        }
        Err(_) => {
            communication.connection_state = super::resource::ConnectionState::Disconnected;
            // 再接続をスケジュール（5秒後）
            communication.next_retry_at = Some(std::time::Instant::now() + Duration::from_secs(5));
            Err("Timeout waiting for TCP connection".to_string())
        }
    }
}

/// ランタイムプロセスを停止
pub fn stop_runtime_process(communication: &mut EditorRuntimeCommunication) -> Result<(), String> {
    if let Some(mut process) = communication.runtime_process.take() {
        process
            .kill()
            .map_err(|e| format!("Failed to kill runtime process: {}", e))?;
        communication.is_separated = false;
        communication.tcp_stream = None;
        communication.connection_state = super::resource::ConnectionState::Disconnected;
        communication.connection_attempts = 0;
        communication.next_retry_at = None;
    }
    Ok(())
}

/// 接続状態をチェックし、必要に応じて再接続を試行
pub fn check_connection_and_reconnect(
    communication: &mut EditorRuntimeCommunication,
    project_path: Option<&std::path::Path>,
) -> Result<(), String> {
    // 既に接続済みなら何もしない
    if communication.connection_state == super::resource::ConnectionState::Connected {
        return Ok(());
    }

    // 分離モードでない場合は何もしない
    if !communication.is_separated {
        return Ok(());
    }

    // 再接続待機中なら時間を確認
    if let Some(next_retry) = communication.next_retry_at {
        if std::time::Instant::now() < next_retry {
            return Ok(()); // まだ再試行しない
        }
    }

    // 再接続を試行
    communication.connection_state = super::resource::ConnectionState::Reconnecting;

    // 既存プロセスが生きているか確認
    let process_alive = if let Some(ref mut process) = communication.runtime_process {
        match process.try_wait() {
            Ok(Some(_)) => false, // プロセスが終了済み
            Ok(None) => true,     // プロセスが実行中
            Err(_) => false,      // エラー時は死んだとみなす
        }
    } else {
        false
    };

    if !process_alive {
        bevy::log::info!("Runtime process not found, restarting...");
        // プロセスが死んでいる場合は再起動
        return start_runtime_process(communication, project_path);
    }

    // プロセスが生きている場合はTCP接続のみ再試行
    if let Some(port) = communication.tcp_port {
        match std::net::TcpStream::connect(format!("127.0.0.1:{}", port)) {
            Ok(stream) => {
                communication.tcp_stream = Some(Arc::new(Mutex::new(stream)));
                communication.connection_state = super::resource::ConnectionState::Connected;
                bevy::log::info!("TCP reconnection successful");
                Ok(())
            }
            Err(e) => {
                communication.connection_state = super::resource::ConnectionState::Disconnected;
                // 再接続をスケジュール（指数バックオフ）
                let backoff_secs = 2_u64.pow(communication.connection_attempts.min(5)); // 最大32秒
                communication.next_retry_at = Some(std::time::Instant::now() + Duration::from_secs(backoff_secs));
                Err(format!("TCP reconnection failed: {}", e))
            }
        }
    } else {
        Err("No TCP port available for reconnection".to_string())
    }
}

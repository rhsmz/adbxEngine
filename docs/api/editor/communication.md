# エディタ通信 API リファレンス

`adbx_editor`クレートのエディタ-ランタイム通信に関するAPIリファレンスです。

## 概要

エディタ-ランタイム通信は、エディタとランタイム間でメッセージを送受信する機能を提供します。同一プロセスモードと分離実行モードの両方をサポートします。

## EditorRuntimeCommunication

エディタ-ランタイム通信リソースです。

### フィールド

- **`editor_tx: Option<Arc<Mutex<mpsc::Sender<EditorMessage>>>>`**: ランタイムへのメッセージ送信チャネル（同一プロセスモード）
- **`runtime_rx: Option<Arc<Mutex<mpsc::Receiver<RuntimeMessage>>>>`**: ランタイムからのメッセージ受信チャネル（同一プロセスモード）
- **`runtime_process: Option<Child>`**: ランタイムプロセス（分離実行モード）
- **`tcp_stream: Option<Arc<Mutex<TcpStream>>>`**: TCP通信ストリーム（分離実行モード）
- **`is_separated: bool`**: 分離実行モードかどうか
- **`tcp_port: Option<u16>`**: TCP通信のポート番号

### 使用例

```rust
use adbx_editor::communication::EditorRuntimeCommunication;

let communication = EditorRuntimeCommunication::default();
```

## ランタイムプロセス管理

### `start_runtime_process`

ランタイムプロセスを起動します（分離実行モード）。

```rust
use adbx_editor::communication::start_runtime_process;

start_runtime_process(&mut communication, Some(&project_path))?;
```

### `stop_runtime_process`

ランタイムプロセスを停止します。

```rust
use adbx_editor::communication::stop_runtime_process;

stop_runtime_process(&mut communication)?;
```

## メッセージ送受信

### `send_to_runtime`

ランタイムにメッセージを送信します。

```rust
use adbx_editor::communication::send_to_runtime;
use adbx_shared::EditorMessage;

send_to_runtime(&communication, EditorMessage::LoadScene {
    scene_path: "scenes/main.scene".to_string(),
})?;
```

### `receive_from_runtime`

ランタイムからメッセージを受信します。

```rust
use adbx_editor::communication::receive_from_runtime;

let messages = receive_from_runtime(&mut communication);
for message in messages {
    match message {
        RuntimeMessage::SceneLoaded { scene } => {
            // シーンが読み込まれた
        }
        _ => {}
    }
}
```

## 通信モード

### 同一プロセスモード

エディタとランタイムが同じプロセスで実行されるモードです。`mpsc`チャネルを使用してメッセージを送受信します。

### 分離実行モード

エディタとランタイムが別々のプロセスで実行されるモードです。TCP通信を使用してメッセージを送受信します。

## 関連型

- [`EditorMessage`](../../shared/protocol.md#editormessage): エディタからランタイムへのメッセージ
- [`RuntimeMessage`](../../shared/protocol.md#runtimemessage): ランタイムからエディタへのメッセージ
